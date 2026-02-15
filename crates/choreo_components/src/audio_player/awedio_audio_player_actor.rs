use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use awedio::backends::CpalBackend;
use awedio::Sound;
use crossbeam_channel::{Receiver, Sender, TrySendError, bounded};

use super::rodio_audio_player_actor::read_duration_seconds;
use crate::audio_player::types::AudioPlayer;

const AUDIO_COMMAND_BUFFER: usize = 1;

#[derive(Clone, Copy)]
struct SharedAudioState {
    is_playing: bool,
    can_seek: bool,
    can_set_speed: bool,
    duration: f64,
    current_position: f64,
}

impl Default for SharedAudioState {
    fn default() -> Self {
        Self {
            is_playing: false,
            can_seek: false,
            can_set_speed: false,
            duration: 0.0,
            current_position: 0.0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
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

struct AudioCommandMailbox {
    sender: Sender<AudioCommand>,
    receiver_probe: Receiver<AudioCommand>,
}

impl AudioCommandMailbox {
    fn new() -> (Self, Receiver<AudioCommand>) {
        let (sender, receiver) = bounded(AUDIO_COMMAND_BUFFER);
        let receiver_probe = receiver.clone();
        (
            Self {
                sender,
                receiver_probe,
            },
            receiver,
        )
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

struct AwedioRuntime {
    manager: Option<awedio::manager::Manager>,
    _backend: Option<CpalBackend>,
    file_path: String,
    duration: f64,
    position: f64,
    is_playing: bool,
    speed: f64,
    volume: f64,
    loop_enabled: bool,
    last_started_at: Option<Instant>,
}

impl AwedioRuntime {
    fn new(file_path: String) -> Self {
        let duration = read_duration_seconds(&file_path);
        let (manager, backend) = match awedio::start() {
            Ok((manager, backend)) => (Some(manager), Some(backend)),
            Err(_) => (None, None),
        };

        Self {
            manager,
            _backend: backend,
            file_path,
            duration,
            position: 0.0,
            is_playing: false,
            speed: 1.0,
            volume: 1.0,
            loop_enabled: false,
            last_started_at: None,
        }
    }

    fn can_seek(&self) -> bool {
        self.manager.is_some()
    }

    fn can_set_speed(&self) -> bool {
        self.manager.is_some()
    }

    fn sync_position(&mut self) {
        if !self.is_playing {
            return;
        }
        let Some(last_started_at) = self.last_started_at else {
            return;
        };

        let elapsed = last_started_at.elapsed().as_secs_f64();
        self.position = clamp_position(self.position + (elapsed * self.speed), self.duration);
        self.last_started_at = Some(Instant::now());

        if self.duration <= 0.0 || self.position < self.duration {
            return;
        }

        if self.loop_enabled {
            self.position = 0.0;
            let _ = self.start_sound_at(0.0);
            self.last_started_at = Some(Instant::now());
            return;
        }

        self.is_playing = false;
        self.last_started_at = None;
        self.clear_manager();
    }

    fn play(&mut self) {
        self.sync_position();
        if self.is_playing {
            return;
        }

        if !self.start_sound_at(self.position) {
            self.is_playing = false;
            self.last_started_at = None;
            return;
        }

        self.is_playing = true;
        self.last_started_at = Some(Instant::now());
    }

    fn pause(&mut self) {
        self.sync_position();
        self.is_playing = false;
        self.last_started_at = None;
        self.clear_manager();
    }

    fn stop(&mut self) {
        self.position = 0.0;
        self.is_playing = false;
        self.last_started_at = None;
        self.clear_manager();
    }

    fn seek(&mut self, position: f64, should_play: bool) {
        self.sync_position();
        self.position = clamp_position(position, self.duration);

        if should_play {
            if !self.start_sound_at(self.position) {
                self.is_playing = false;
                self.last_started_at = None;
                return;
            }
            self.is_playing = true;
            self.last_started_at = Some(Instant::now());
            return;
        }

        self.is_playing = false;
        self.last_started_at = None;
        self.clear_manager();
    }

    fn set_speed(&mut self, speed: f64) {
        self.sync_position();
        self.speed = speed.clamp(0.5, 2.0);
        if self.is_playing {
            let current = self.position;
            if !self.start_sound_at(current) {
                self.is_playing = false;
                self.last_started_at = None;
                return;
            }
            self.last_started_at = Some(Instant::now());
        }
    }

    fn set_volume(&mut self, volume: f64) {
        self.volume = volume.clamp(0.0, 1.0);
        if self.is_playing {
            let current = self.position;
            if !self.start_sound_at(current) {
                self.is_playing = false;
                self.last_started_at = None;
                return;
            }
            self.last_started_at = Some(Instant::now());
        }
    }

    fn start_sound_at(&mut self, start_seconds: f64) -> bool {
        let Some(manager) = self.manager.as_mut() else {
            return false;
        };

        manager.clear();

        let Ok(mut sound) = awedio::sounds::open_file(self.file_path.as_str()) else {
            return false;
        };

        if start_seconds > 0.0 {
            let _ = sound.skip(Duration::from_secs_f64(start_seconds));
        }

        let sound = sound.with_adjustable_speed_of(self.speed as f32);
        let sound = sound.with_adjustable_volume_of(self.volume as f32);
        manager.play(Box::new(sound));
        true
    }

    fn clear_manager(&mut self) {
        if let Some(manager) = self.manager.as_mut() {
            manager.clear();
        }
    }

    fn handle(&mut self, command: AudioCommand) {
        match command {
            AudioCommand::Play => self.play(),
            AudioCommand::Pause => self.pause(),
            AudioCommand::Stop => self.stop(),
            AudioCommand::Seek(position) => self.seek(position, false),
            AudioCommand::SeekAndPlay(position) => self.seek(position, true),
            AudioCommand::SetSpeed(speed) => self.set_speed(speed),
            AudioCommand::SetVolume(volume) => self.set_volume(volume),
            AudioCommand::SetBalance(_balance) => {}
            AudioCommand::SetLoop(loop_enabled) => {
                self.loop_enabled = loop_enabled;
            }
            AudioCommand::Shutdown => {}
        }
    }

    fn tick(&mut self) {
        self.sync_position();
    }

    fn publish(&self, shared: &Arc<Mutex<SharedAudioState>>) {
        if let Ok(mut state) = shared.lock() {
            state.is_playing = self.is_playing;
            state.can_seek = self.can_seek();
            state.can_set_speed = self.can_set_speed();
            state.duration = self.duration;
            state.current_position = self.position;
        }
    }
}

pub(super) struct AwedioAudioPlayerActor {
    shared: Arc<Mutex<SharedAudioState>>,
    mailbox: AudioCommandMailbox,
    worker: Option<JoinHandle<()>>,
}

impl AwedioAudioPlayerActor {
    pub(super) fn new(file_path: String) -> Self {
        let shared = Arc::new(Mutex::new(SharedAudioState::default()));
        let shared_for_worker = Arc::clone(&shared);
        let (mailbox, receiver) = AudioCommandMailbox::new();
        let worker = thread::Builder::new()
            .name("audio-player-actor-awedio".to_string())
            .spawn(move || {
                let mut runtime = AwedioRuntime::new(file_path);
                runtime.publish(&shared_for_worker);
                loop {
                    match receiver.recv_timeout(Duration::from_millis(16)) {
                        Ok(command) => {
                            let mut pending_commands = PendingAudioCommands::default();
                            let mut seq: usize = 0;
                            pending_commands.merge(seq, command);
                            seq += 1;

                            while let Ok(queued) = receiver.try_recv() {
                                pending_commands.merge(seq, queued);
                                seq += 1;
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
                    runtime.publish(&shared_for_worker);
                }
            })
            .ok();

        Self {
            shared,
            mailbox,
            worker,
        }
    }

    fn state(&self) -> SharedAudioState {
        self.shared.lock().map(|state| *state).unwrap_or_default()
    }
}

impl AudioPlayer for AwedioAudioPlayerActor {
    fn is_playing(&self) -> bool {
        self.state().is_playing
    }

    fn can_seek(&self) -> bool {
        self.state().can_seek
    }

    fn can_set_speed(&self) -> bool {
        self.state().can_set_speed
    }

    fn duration(&self) -> f64 {
        self.state().duration
    }

    fn current_position(&self) -> f64 {
        self.state().current_position
    }

    fn play(&mut self) {
        self.mailbox.send_latest(AudioCommand::Play);
    }

    fn pause(&mut self) {
        self.mailbox.send_latest(AudioCommand::Pause);
    }

    fn stop(&mut self) {
        self.mailbox.send_latest(AudioCommand::Stop);
    }

    fn seek(&mut self, position: f64) {
        self.mailbox.send_latest(AudioCommand::Seek(position));
    }

    fn seek_and_play(&mut self, position: f64) {
        self.mailbox
            .send_latest(AudioCommand::SeekAndPlay(position));
    }

    fn set_speed(&mut self, speed: f64) {
        self.mailbox.send_latest(AudioCommand::SetSpeed(speed));
    }

    fn set_volume(&mut self, volume: f64) {
        self.mailbox.send_latest(AudioCommand::SetVolume(volume));
    }

    fn set_balance(&mut self, balance: f64) {
        self.mailbox.send_latest(AudioCommand::SetBalance(balance));
    }

    fn set_loop(&mut self, loop_enabled: bool) {
        self.mailbox.send_latest(AudioCommand::SetLoop(loop_enabled));
    }
}

impl Drop for AwedioAudioPlayerActor {
    fn drop(&mut self) {
        self.mailbox.send_latest(AudioCommand::Shutdown);
        if let Some(worker) = self.worker.take() {
            let _ = worker.join();
        }
    }
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

#[cfg(test)]
mod tests {
    use super::{AudioCommand, AudioCommandMailbox, PendingAudioCommands};

    #[test]
    fn command_mailbox_keeps_latest_when_queue_is_full() {
        let (mailbox, receiver) = AudioCommandMailbox::new();
        mailbox.send_latest(AudioCommand::SetSpeed(1.0));
        mailbox.send_latest(AudioCommand::SetSpeed(1.5));

        let queued = receiver.try_recv().expect("latest command should be queued");
        assert_eq!(queued, AudioCommand::SetSpeed(1.5));
    }

    #[test]
    fn pending_commands_coalesce_by_type_and_preserve_order() {
        let mut pending = PendingAudioCommands::default();
        pending.merge(0, AudioCommand::SetSpeed(1.0));
        pending.merge(1, AudioCommand::SetSpeed(1.5));
        pending.merge(2, AudioCommand::Seek(3.0));
        pending.merge(3, AudioCommand::Seek(4.0));
        pending.merge(4, AudioCommand::Play);

        let ordered = pending.into_ordered_commands();
        assert_eq!(
            ordered,
            vec![
                AudioCommand::SetSpeed(1.5),
                AudioCommand::Seek(4.0),
                AudioCommand::Play
            ],
        );
    }
}
