use std::time::Instant;

use super::super::types::AudioPlayer;
use super::super::types::AudioPlayerSample;

pub(super) struct NativeAudioPlayerActor {
    duration: f64,
    accumulated_position: f64,
    started_at: Option<Instant>,
    speed: f64,
    volume: f64,
    balance: f64,
    loop_enabled: bool,
}

impl NativeAudioPlayerActor {
    #[must_use]
    pub(super) fn new(file_path: String) -> Self {
        let duration = if file_path.trim().is_empty() {
            0.0
        } else {
            1.0
        };
        Self {
            duration,
            accumulated_position: 0.0,
            started_at: None,
            speed: 1.0,
            volume: 1.0,
            balance: 0.0,
            loop_enabled: false,
        }
    }

    fn sync_position_to_now(&mut self) {
        let Some(started_at) = self.started_at else {
            return;
        };

        let elapsed = started_at.elapsed().as_secs_f64() * self.speed.max(0.0);
        let mut next = self.accumulated_position + elapsed;
        if self.duration > 0.0 {
            if self.loop_enabled {
                next %= self.duration;
            } else if next > self.duration {
                next = self.duration;
                self.started_at = None;
            }
        }

        self.accumulated_position = next;
        if self.started_at.is_some() {
            self.started_at = Some(Instant::now());
        }
    }
}

impl AudioPlayer for NativeAudioPlayerActor {
    fn sample(&self) -> AudioPlayerSample {
        AudioPlayerSample {
            is_playing: self.started_at.is_some(),
            can_seek: self.duration > 0.0,
            can_set_speed: true,
            duration: self.duration,
            position: self.accumulated_position,
            speed: self.speed,
            volume: self.volume,
            balance: self.balance,
            loop_enabled: self.loop_enabled,
        }
    }

    fn play(&mut self) {
        if self.started_at.is_none() {
            self.started_at = Some(Instant::now());
        }
    }

    fn pause(&mut self) {
        self.sync_position_to_now();
        self.started_at = None;
    }

    fn stop(&mut self) {
        self.started_at = None;
        self.accumulated_position = 0.0;
    }

    fn seek(&mut self, position: f64) {
        self.sync_position_to_now();
        self.accumulated_position = if self.duration > 0.0 {
            position.clamp(0.0, self.duration)
        } else {
            0.0
        };
    }

    fn set_speed(&mut self, speed: f64) {
        self.sync_position_to_now();
        self.speed = speed.max(0.0);
    }

    fn set_volume(&mut self, volume: f64) {
        self.volume = volume.clamp(0.0, 1.0);
    }

    fn set_balance(&mut self, balance: f64) {
        self.balance = balance.clamp(-1.0, 1.0);
    }

    fn set_loop(&mut self, loop_enabled: bool) {
        self.loop_enabled = loop_enabled;
    }
}
