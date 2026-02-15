use std::sync::Mutex;
use std::time::{Duration, Instant};

use awedio::backends::CpalBackend;
use awedio::Sound;

use super::rodio_audio_player_actor::read_duration_seconds;
use crate::audio_player::types::AudioPlayer;

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
}

pub(super) struct AwedioAudioPlayerActor {
    runtime: Mutex<AwedioRuntime>,
}

impl AwedioAudioPlayerActor {
    pub(super) fn new(file_path: String) -> Self {
        Self {
            runtime: Mutex::new(AwedioRuntime::new(file_path)),
        }
    }
}

impl AudioPlayer for AwedioAudioPlayerActor {
    fn is_playing(&self) -> bool {
        let Ok(mut runtime) = self.runtime.lock() else {
            return false;
        };
        runtime.sync_position();
        runtime.is_playing
    }

    fn can_seek(&self) -> bool {
        self.runtime
            .lock()
            .map(|runtime| runtime.can_seek())
            .unwrap_or(false)
    }

    fn can_set_speed(&self) -> bool {
        self.runtime
            .lock()
            .map(|runtime| runtime.can_set_speed())
            .unwrap_or(false)
    }

    fn duration(&self) -> f64 {
        self.runtime
            .lock()
            .map(|runtime| runtime.duration)
            .unwrap_or(0.0)
    }

    fn current_position(&self) -> f64 {
        let Ok(mut runtime) = self.runtime.lock() else {
            return 0.0;
        };
        runtime.sync_position();
        runtime.position
    }

    fn play(&mut self) {
        if let Ok(mut runtime) = self.runtime.lock() {
            runtime.play();
        }
    }

    fn pause(&mut self) {
        if let Ok(mut runtime) = self.runtime.lock() {
            runtime.pause();
        }
    }

    fn stop(&mut self) {
        if let Ok(mut runtime) = self.runtime.lock() {
            runtime.stop();
        }
    }

    fn seek(&mut self, position: f64) {
        if let Ok(mut runtime) = self.runtime.lock() {
            runtime.seek(position, false);
        }
    }

    fn seek_and_play(&mut self, position: f64) {
        if let Ok(mut runtime) = self.runtime.lock() {
            runtime.seek(position, true);
        }
    }

    fn set_speed(&mut self, speed: f64) {
        if let Ok(mut runtime) = self.runtime.lock() {
            runtime.set_speed(speed);
        }
    }

    fn set_volume(&mut self, volume: f64) {
        if let Ok(mut runtime) = self.runtime.lock() {
            runtime.set_volume(volume);
        }
    }

    fn set_balance(&mut self, _balance: f64) {}

    fn set_loop(&mut self, loop_enabled: bool) {
        if let Ok(mut runtime) = self.runtime.lock() {
            runtime.loop_enabled = loop_enabled;
        }
    }
}

fn clamp_position(position: f64, duration: f64) -> f64 {
    if duration <= 0.0 {
        return position.max(0.0);
    }
    position.clamp(0.0, duration)
}
