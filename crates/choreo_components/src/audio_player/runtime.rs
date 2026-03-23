use super::audio_player_actor::create_platform_audio_player;
use super::audio_player_backend::AudioPlayerBackend;
use super::state::AudioPlayerState;
use super::state::duration_label;
use super::state::speed_to_percent_text;
use super::types::AudioPlayer;
use super::types::AudioPlayerSample;

pub struct AudioPlayerRuntime {
    backend: AudioPlayerBackend,
    player: Option<Box<dyn AudioPlayer>>,
}

impl AudioPlayerRuntime {
    #[must_use]
    pub fn new(backend: AudioPlayerBackend) -> Self {
        Self {
            backend,
            player: None,
        }
    }

    pub fn open_file(&mut self, file_path: String) {
        self.player = Some(create_platform_audio_player(file_path, self.backend));
    }

    pub fn set_backend(&mut self, backend: AudioPlayerBackend) {
        if self.backend == backend {
            return;
        }
        self.backend = backend;
        self.player = None;
    }

    pub fn close(&mut self) {
        self.player = None;
    }

    #[must_use]
    pub fn has_player(&self) -> bool {
        self.player.is_some()
    }

    pub fn sample(&self) -> Option<AudioPlayerSample> {
        self.player.as_ref().map(|player| player.sample())
    }

    pub fn toggle_play_pause(&mut self) {
        let Some(player) = self.player.as_mut() else {
            return;
        };
        if player.sample().is_playing {
            player.pause();
        } else {
            player.play();
        }
    }

    pub fn pause(&mut self) {
        if let Some(player) = self.player.as_mut() {
            player.pause();
        }
    }

    pub fn stop(&mut self) {
        if let Some(player) = self.player.as_mut() {
            player.stop();
        }
    }

    pub fn seek(&mut self, position: f64) {
        if let Some(player) = self.player.as_mut() {
            player.seek(position);
        }
    }

    pub fn seek_and_play(&mut self, position: f64) {
        if let Some(player) = self.player.as_mut() {
            player.seek_and_play(position);
        }
    }

    pub fn set_speed(&mut self, speed: f64) {
        if let Some(player) = self.player.as_mut() {
            player.set_speed(speed);
        }
    }

    pub fn set_volume(&mut self, volume: f64) {
        if let Some(player) = self.player.as_mut() {
            player.set_volume(volume);
        }
    }

    pub fn set_balance(&mut self, balance: f64) {
        if let Some(player) = self.player.as_mut() {
            player.set_balance(balance);
        }
    }

    pub fn set_loop(&mut self, loop_enabled: bool) {
        if let Some(player) = self.player.as_mut() {
            player.set_loop(loop_enabled);
        }
    }
}

pub fn apply_player_sample(state: &mut AudioPlayerState, sample: AudioPlayerSample) {
    apply_player_sample_inner(state, sample, true);
}

pub fn apply_player_sample_without_position(
    state: &mut AudioPlayerState,
    sample: AudioPlayerSample,
) {
    apply_player_sample_inner(state, sample, false);
}

fn apply_player_sample_inner(
    state: &mut AudioPlayerState,
    sample: AudioPlayerSample,
    include_position: bool,
) {
    state.is_playing = sample.is_playing;
    state.can_seek = sample.can_seek;
    state.can_set_speed = sample.can_set_speed;
    state.duration = sample.duration;
    if include_position {
        state.position = sample.position;
    }
    state.speed = sample.speed;
    state.volume = sample.volume;
    state.balance = sample.balance;
    state.loop_enabled = sample.loop_enabled;
    state.has_player = true;
    state.speed_label = speed_to_percent_text(state.speed);
    state.duration_label = duration_label(state.position, state.duration);
}
