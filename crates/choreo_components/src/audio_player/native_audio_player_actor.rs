use std::fs::File;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use std::time::Instant;

use crossbeam_channel::Receiver;
use crossbeam_channel::Sender;
use crossbeam_channel::TrySendError;
use crossbeam_channel::bounded;
use rodio::Decoder;
use rodio::DeviceSinkBuilder;
use rodio::MixerDeviceSink;
use rodio::Player;
use rodio::Source;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;
use symphonia::default::get_probe;

use super::super::types::AudioPlayer;
use super::super::types::AudioPlayerSample;

const AUDIO_COMMAND_BUFFER: usize = 1;

#[derive(Clone, Copy)]
struct SharedAudioState {
    is_playing: bool,
    can_seek: bool,
    can_set_speed: bool,
    duration: f64,
    current_position: f64,
    speed: f64,
    volume: f64,
    balance: f64,
    loop_enabled: bool,
}

impl Default for SharedAudioState {
    fn default() -> Self {
        Self {
            is_playing: false,
            can_seek: true,
            can_set_speed: true,
            duration: 0.0,
            current_position: 0.0,
            speed: 1.0,
            volume: 1.0,
            balance: 0.0,
            loop_enabled: false,
        }
    }
}

enum AudioCommand {
    Play,
    Pause,
    Stop,
    Seek(f64),
    SeekAndPlay(f64),
    SetSpeed(f64),
    SetVolume(f64),
    SetBalance(f64),
    SetLoop(bool),
    Shutdown,
}

pub(super) struct NativeAudioPlayerActor {
    shared: Arc<Mutex<SharedAudioState>>,
    sender: Sender<AudioCommand>,
    receiver_probe: Receiver<AudioCommand>,
    worker: Option<JoinHandle<()>>,
}

impl NativeAudioPlayerActor {
    #[must_use]
    pub(super) fn new(file_path: String) -> Self {
        let shared = Arc::new(Mutex::new(SharedAudioState::default()));
        let (sender, receiver) = bounded(AUDIO_COMMAND_BUFFER);
        let receiver_probe = receiver.clone();
        let shared_for_thread = Arc::clone(&shared);
        let worker = thread::Builder::new()
            .name("audio-player-actor-native".to_string())
            .spawn(move || {
                let mut runtime = NativeRuntime::new(file_path);
                runtime.publish(&shared_for_thread);
                loop {
                    match receiver.recv_timeout(Duration::from_millis(16)) {
                        Ok(command) => {
                            let mut pending_commands = PendingAudioCommands::default();
                            let mut sequence: usize = 0;

                            pending_commands.merge(sequence, command);
                            sequence += 1;

                            while let Ok(queued) = receiver.try_recv() {
                                pending_commands.merge(sequence, queued);
                                sequence += 1;
                            }

                            if pending_commands.has_shutdown() {
                                break;
                            }

                            for pending_command in pending_commands.into_ordered_commands() {
                                runtime.handle(pending_command);
                            }
                        }
                        Err(crossbeam_channel::RecvTimeoutError::Timeout) => {}
                        Err(crossbeam_channel::RecvTimeoutError::Disconnected) => break,
                    }

                    runtime.tick();
                    runtime.publish(&shared_for_thread);
                }
            })
            .ok();

        Self {
            shared,
            sender,
            receiver_probe,
            worker,
        }
    }

    fn state(&self) -> SharedAudioState {
        self.shared.lock().map(|state| *state).unwrap_or_default()
    }

    fn send_latest(&self, command: AudioCommand) {
        match self.sender.try_send(command) {
            Ok(()) => {}
            Err(TrySendError::Full(command)) => {
                let _ = self.receiver_probe.try_recv();
                let _ = self.sender.try_send(command);
            }
            Err(TrySendError::Disconnected(_)) => {}
        }
    }
}

impl AudioPlayer for NativeAudioPlayerActor {
    fn sample(&self) -> AudioPlayerSample {
        let state = self.state();
        AudioPlayerSample {
            is_playing: state.is_playing,
            can_seek: state.can_seek,
            can_set_speed: state.can_set_speed,
            duration: state.duration,
            position: state.current_position,
            speed: state.speed,
            volume: state.volume,
            balance: state.balance,
            loop_enabled: state.loop_enabled,
        }
    }

    fn play(&mut self) {
        self.send_latest(AudioCommand::Play);
    }

    fn pause(&mut self) {
        self.send_latest(AudioCommand::Pause);
    }

    fn stop(&mut self) {
        self.send_latest(AudioCommand::Stop);
    }

    fn seek(&mut self, position: f64) {
        self.send_latest(AudioCommand::Seek(position));
    }

    fn seek_and_play(&mut self, position: f64) {
        self.send_latest(AudioCommand::SeekAndPlay(position));
    }

    fn set_speed(&mut self, speed: f64) {
        self.send_latest(AudioCommand::SetSpeed(speed));
    }

    fn set_volume(&mut self, volume: f64) {
        self.send_latest(AudioCommand::SetVolume(volume));
    }

    fn set_balance(&mut self, balance: f64) {
        self.send_latest(AudioCommand::SetBalance(balance));
    }

    fn set_loop(&mut self, loop_enabled: bool) {
        self.send_latest(AudioCommand::SetLoop(loop_enabled));
    }
}

impl Drop for NativeAudioPlayerActor {
    fn drop(&mut self) {
        loop {
            match self.sender.try_send(AudioCommand::Shutdown) {
                Ok(()) => break,
                Err(TrySendError::Full(_)) => {
                    let _ = self.receiver_probe.try_recv();
                }
                Err(TrySendError::Disconnected(_)) => break,
            }
        }

        if let Some(worker) = self.worker.take() {
            let _ = worker.join();
        }
    }
}

enum Engine {
    Rodio {
        _device_sink: MixerDeviceSink,
        player: Player,
    },
    Silent,
}

impl Engine {
    fn play(&self) {
        if let Self::Rodio { player, .. } = self {
            player.play();
        }
    }

    fn pause(&self) {
        if let Self::Rodio { player, .. } = self {
            player.pause();
        }
    }

    fn set_speed(&self, speed: f64) {
        if let Self::Rodio { player, .. } = self {
            player.set_speed(speed as f32);
        }
    }

    fn set_volume(&self, volume: f64) {
        if let Self::Rodio { player, .. } = self {
            player.set_volume(volume as f32);
        }
    }

    fn try_seek(&self, position: Duration) -> bool {
        match self {
            Self::Rodio { player, .. } => player.try_seek(position).is_ok(),
            Self::Silent => false,
        }
    }

    fn position(&self) -> Option<f64> {
        match self {
            Self::Rodio { player, .. } => Some(player.get_pos().as_secs_f64()),
            Self::Silent => None,
        }
    }

    fn is_empty(&self) -> bool {
        match self {
            Self::Rodio { player, .. } => player.empty(),
            Self::Silent => false,
        }
    }

    fn is_available(&self) -> bool {
        matches!(self, Self::Rodio { .. })
    }
}

struct NativeRuntime {
    file_path: String,
    duration: f64,
    position: f64,
    is_playing: bool,
    speed: f64,
    volume: f64,
    balance: f64,
    loop_enabled: bool,
    last_started_at: Option<Instant>,
    engine: Engine,
}

impl NativeRuntime {
    fn new(file_path: String) -> Self {
        let duration = read_duration_seconds(&file_path);
        let mut runtime = Self {
            file_path,
            duration,
            position: 0.0,
            is_playing: false,
            speed: 1.0,
            volume: 1.0,
            balance: 0.0,
            loop_enabled: false,
            last_started_at: None,
            engine: Engine::Silent,
        };
        runtime.rebuild_sink(0.0, false);
        runtime
    }

    fn handle(&mut self, command: AudioCommand) {
        match command {
            AudioCommand::Play => {
                if !self.is_playing {
                    self.is_playing = true;
                    self.last_started_at = Some(Instant::now());
                    self.engine.play();
                }
            }
            AudioCommand::Pause => {
                self.sync_position();
                self.is_playing = false;
                self.last_started_at = None;
                self.engine.pause();
            }
            AudioCommand::Stop => {
                self.position = 0.0;
                self.is_playing = false;
                self.last_started_at = None;
                if self.engine.try_seek(Duration::ZERO) {
                    self.engine.pause();
                } else {
                    self.rebuild_sink(0.0, false);
                }
            }
            AudioCommand::Seek(position) => {
                self.sync_position();
                self.position = clamp_position(position, self.duration);
                if !self
                    .engine
                    .try_seek(Duration::from_secs_f64(self.position.max(0.0)))
                {
                    self.rebuild_sink(self.position, self.is_playing);
                }
                self.last_started_at = if self.is_playing {
                    Some(Instant::now())
                } else {
                    None
                };
            }
            AudioCommand::SeekAndPlay(position) => {
                self.sync_position();
                self.position = clamp_position(position, self.duration);
                self.is_playing = true;
                if !self
                    .engine
                    .try_seek(Duration::from_secs_f64(self.position.max(0.0)))
                {
                    self.rebuild_sink(self.position, true);
                } else {
                    self.engine.play();
                }
                self.last_started_at = Some(Instant::now());
            }
            AudioCommand::SetSpeed(speed) => {
                self.sync_position();
                self.speed = speed.clamp(0.5, 2.0);
                self.engine.set_speed(self.speed);
                if self.is_playing {
                    self.last_started_at = Some(Instant::now());
                }
            }
            AudioCommand::SetVolume(volume) => {
                self.volume = volume.clamp(0.0, 1.0);
                self.engine.set_volume(self.volume);
            }
            AudioCommand::SetBalance(balance) => {
                self.balance = balance.clamp(-1.0, 1.0);
            }
            AudioCommand::SetLoop(loop_enabled) => {
                self.loop_enabled = loop_enabled;
            }
            AudioCommand::Shutdown => {}
        }
    }

    fn tick(&mut self) {
        self.sync_position();
        if !self.is_playing {
            return;
        }

        let reached_end = if self.duration > 0.0 {
            self.position >= self.duration
        } else if self.engine.is_available() {
            self.engine.is_empty()
        } else {
            false
        };
        if !reached_end {
            return;
        }

        if self.loop_enabled {
            self.position = 0.0;
            if !self.engine.try_seek(Duration::ZERO) {
                self.rebuild_sink(0.0, true);
            } else {
                self.engine.play();
            }
            self.last_started_at = Some(Instant::now());
            return;
        }

        self.is_playing = false;
        self.last_started_at = None;
        self.position = self.duration.max(0.0);
        self.engine.pause();
    }

    fn publish(&self, shared: &Arc<Mutex<SharedAudioState>>) {
        if let Ok(mut state) = shared.lock() {
            state.is_playing = self.is_playing;
            state.can_seek = true;
            state.can_set_speed = true;
            state.duration = self.duration;
            state.current_position = self.position;
            state.speed = self.speed;
            state.volume = self.volume;
            state.balance = self.balance;
            state.loop_enabled = self.loop_enabled;
        }
    }

    fn sync_position(&mut self) {
        if let Some(position) = self.engine.position() {
            self.position = clamp_position(position, self.duration);
            if self.is_playing {
                self.last_started_at = Some(Instant::now());
            }
            return;
        }

        if !self.is_playing {
            return;
        }
        let Some(last_started_at) = self.last_started_at else {
            return;
        };
        let elapsed = last_started_at.elapsed().as_secs_f64();
        self.position = clamp_position(self.position + (elapsed * self.speed), self.duration);
        self.last_started_at = Some(Instant::now());
    }

    fn rebuild_sink(&mut self, start_seconds: f64, should_play: bool) {
        let Some(device_sink) = DeviceSinkBuilder::open_default_sink().ok() else {
            self.engine = Engine::Silent;
            return;
        };

        let player = Player::connect_new(device_sink.mixer());
        let loaded = append_source(&player, &self.file_path, start_seconds);
        if !loaded {
            self.engine = Engine::Silent;
            return;
        }

        player.set_volume(self.volume as f32);
        player.set_speed(self.speed as f32);
        if should_play {
            player.play();
        } else {
            player.pause();
        }

        self.engine = Engine::Rodio {
            _device_sink: device_sink,
            player,
        };
    }
}

fn append_source(player: &Player, file_path: &str, start_seconds: f64) -> bool {
    let Some(file) = File::open(file_path).ok() else {
        return false;
    };
    let Some(decoder) = Decoder::try_from(file).ok() else {
        return false;
    };

    if start_seconds > 0.0 {
        player.append(decoder.skip_duration(Duration::from_secs_f64(start_seconds)));
    } else {
        player.append(decoder);
    }
    true
}

fn read_duration_seconds(file_path: &str) -> f64 {
    let Some(file) = File::open(file_path).ok() else {
        return 0.0;
    };
    let Some(decoder) = Decoder::try_from(file).ok() else {
        return read_duration_seconds_with_symphonia(file_path);
    };
    let rodio_duration = decoder
        .total_duration()
        .map(|value| value.as_secs_f64())
        .unwrap_or(0.0);
    if rodio_duration > 0.0 {
        rodio_duration
    } else {
        read_duration_seconds_with_symphonia(file_path)
    }
}

fn read_duration_seconds_with_symphonia(file_path: &str) -> f64 {
    let Some(file) = File::open(file_path).ok() else {
        return 0.0;
    };
    let mss = MediaSourceStream::new(Box::new(file), Default::default());
    let mut hint = Hint::new();
    if let Some(extension) = std::path::Path::new(file_path)
        .extension()
        .and_then(|value| value.to_str())
    {
        hint.with_extension(extension);
    }

    let Ok(probed) = get_probe().format(
        &hint,
        mss,
        &FormatOptions::default(),
        &MetadataOptions::default(),
    ) else {
        return 0.0;
    };
    let track = match probed.format.default_track() {
        Some(track) => track,
        None => return 0.0,
    };

    let sample_rate = match track.codec_params.sample_rate {
        Some(sample_rate) if sample_rate > 0 => sample_rate,
        _ => return 0.0,
    };
    let n_frames = match track.codec_params.n_frames {
        Some(n_frames) if n_frames > 0 => n_frames,
        _ => return 0.0,
    };

    n_frames as f64 / sample_rate as f64
}

fn clamp_position(position: f64, duration: f64) -> f64 {
    if duration <= 0.0 {
        return position.max(0.0);
    }
    position.clamp(0.0, duration)
}

#[derive(Default)]
struct PendingAudioCommands {
    latest_control: Option<(usize, AudioCommand)>,
    latest_seek: Option<(usize, f64)>,
    latest_seek_and_play: Option<(usize, f64)>,
    latest_speed: Option<(usize, f64)>,
    latest_volume: Option<(usize, f64)>,
    latest_balance: Option<(usize, f64)>,
    latest_loop: Option<(usize, bool)>,
}

impl PendingAudioCommands {
    fn merge(&mut self, sequence: usize, command: AudioCommand) {
        match command {
            AudioCommand::Play
            | AudioCommand::Pause
            | AudioCommand::Stop
            | AudioCommand::Shutdown => {
                self.latest_control = Some((sequence, command));
            }
            AudioCommand::Seek(position) => {
                self.latest_seek = Some((sequence, position));
            }
            AudioCommand::SeekAndPlay(position) => {
                self.latest_seek_and_play = Some((sequence, position));
            }
            AudioCommand::SetSpeed(speed) => {
                self.latest_speed = Some((sequence, speed));
            }
            AudioCommand::SetVolume(volume) => {
                self.latest_volume = Some((sequence, volume));
            }
            AudioCommand::SetBalance(balance) => {
                self.latest_balance = Some((sequence, balance));
            }
            AudioCommand::SetLoop(loop_enabled) => {
                self.latest_loop = Some((sequence, loop_enabled));
            }
        }
    }

    fn has_shutdown(&self) -> bool {
        matches!(self.latest_control, Some((_, AudioCommand::Shutdown)))
    }

    fn into_ordered_commands(self) -> Vec<AudioCommand> {
        let mut pending: Vec<(usize, AudioCommand)> = Vec::new();
        if let Some((order, control)) = self.latest_control {
            pending.push((order, control));
        }
        if let Some((order, position)) = self.latest_seek {
            pending.push((order, AudioCommand::Seek(position)));
        }
        if let Some((order, position)) = self.latest_seek_and_play {
            pending.push((order, AudioCommand::SeekAndPlay(position)));
        }
        if let Some((order, speed)) = self.latest_speed {
            pending.push((order, AudioCommand::SetSpeed(speed)));
        }
        if let Some((order, volume)) = self.latest_volume {
            pending.push((order, AudioCommand::SetVolume(volume)));
        }
        if let Some((order, balance)) = self.latest_balance {
            pending.push((order, AudioCommand::SetBalance(balance)));
        }
        if let Some((order, loop_enabled)) = self.latest_loop {
            pending.push((order, AudioCommand::SetLoop(loop_enabled)));
        }
        pending.sort_by_key(|(order, _)| *order);
        pending.into_iter().map(|(_, command)| command).collect()
    }
}
