use super::super::types::AudioPlayer;
use super::super::types::AudioPlayerSample;

pub struct BrowserAudioPlayerActor {
    sample: AudioPlayerSample,
}

impl BrowserAudioPlayerActor {
    #[must_use]
    pub fn new(file_path: String) -> Self {
        let duration = if file_path.trim().is_empty() { 0.0 } else { 1.0 };
        Self {
            sample: AudioPlayerSample {
                is_playing: false,
                can_seek: duration > 0.0,
                can_set_speed: true,
                duration,
                position: 0.0,
                speed: 1.0,
                volume: 1.0,
                balance: 0.0,
                loop_enabled: false,
            },
        }
    }
}

impl AudioPlayer for BrowserAudioPlayerActor {
    fn sample(&self) -> AudioPlayerSample {
        self.sample
    }

    fn play(&mut self) {
        self.sample.is_playing = true;
    }

    fn pause(&mut self) {
        self.sample.is_playing = false;
    }

    fn stop(&mut self) {
        self.sample.is_playing = false;
        self.sample.position = 0.0;
    }

    fn seek(&mut self, position: f64) {
        self.sample.position = if self.sample.duration > 0.0 {
            position.clamp(0.0, self.sample.duration)
        } else {
            0.0
        };
    }

    fn set_speed(&mut self, speed: f64) {
        self.sample.speed = speed.max(0.0);
    }

    fn set_volume(&mut self, volume: f64) {
        self.sample.volume = volume.clamp(0.0, 1.0);
    }

    fn set_balance(&mut self, balance: f64) {
        self.sample.balance = balance.clamp(-1.0, 1.0);
    }

    fn set_loop(&mut self, loop_enabled: bool) {
        self.sample.loop_enabled = loop_enabled;
    }
}
