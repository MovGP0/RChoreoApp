use super::super::types::AudioPlayer;
use super::super::types::AudioPlayerSample;

pub(super) struct SilentAudioPlayerActor {
    sample: AudioPlayerSample,
}

impl SilentAudioPlayerActor {
    #[must_use]
    pub(super) fn new() -> Self {
        Self {
            sample: AudioPlayerSample {
                is_playing: false,
                can_seek: false,
                can_set_speed: false,
                duration: 0.0,
                position: 0.0,
                speed: 1.0,
                volume: 1.0,
                balance: 0.0,
                loop_enabled: false,
            },
        }
    }
}

impl AudioPlayer for SilentAudioPlayerActor {
    fn sample(&self) -> AudioPlayerSample {
        self.sample
    }

    fn play(&mut self) {}

    fn pause(&mut self) {}

    fn stop(&mut self) {}

    fn seek(&mut self, _position: f64) {}

    fn set_speed(&mut self, _speed: f64) {}

    fn set_volume(&mut self, _volume: f64) {}

    fn set_balance(&mut self, _balance: f64) {}

    fn set_loop(&mut self, loop_enabled: bool) {
        self.sample.loop_enabled = loop_enabled;
    }
}
