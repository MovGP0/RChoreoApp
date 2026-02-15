use web_sys::HtmlAudioElement;

use crate::audio_player::types::AudioPlayer;

#[derive(Clone, Copy)]
struct SharedAudioState {
    is_playing: bool,
    duration: f64,
    current_position: f64,
}

impl Default for SharedAudioState {
    fn default() -> Self {
        Self {
            is_playing: false,
            duration: 0.0,
            current_position: 0.0,
        }
    }
}

pub(super) struct BrowserAudioPlayerActor {
    audio: Option<HtmlAudioElement>,
    state: SharedAudioState,
}

impl BrowserAudioPlayerActor {
    pub(super) fn new(file_path: String) -> Self {
        let audio = HtmlAudioElement::new_with_src(&file_path).ok();
        let mut state = SharedAudioState::default();
        if let Some(element) = audio.as_ref() {
            state.duration = element.duration().max(0.0);
            state.current_position = element.current_time().max(0.0);
        }
        Self { audio, state }
    }

    fn sync_state(&mut self) {
        let Some(audio) = self.audio.as_ref() else {
            return;
        };

        self.state.is_playing = !audio.paused();
        self.state.duration = audio.duration().max(0.0);
        self.state.current_position = audio.current_time().max(0.0);
    }
}

impl AudioPlayer for BrowserAudioPlayerActor {
    fn is_playing(&self) -> bool {
        self.audio
            .as_ref()
            .map(|audio| !audio.paused())
            .unwrap_or(self.state.is_playing)
    }

    fn can_seek(&self) -> bool {
        self.audio.is_some()
    }

    fn can_set_speed(&self) -> bool {
        self.audio.is_some()
    }

    fn duration(&self) -> f64 {
        self.audio
            .as_ref()
            .map(|audio| audio.duration().max(0.0))
            .unwrap_or(self.state.duration)
    }

    fn current_position(&self) -> f64 {
        self.audio
            .as_ref()
            .map(|audio| audio.current_time().max(0.0))
            .unwrap_or(self.state.current_position)
    }

    fn play(&mut self) {
        if let Some(audio) = self.audio.as_ref() {
            let _ = audio.play();
        }
        self.sync_state();
    }

    fn pause(&mut self) {
        if let Some(audio) = self.audio.as_ref() {
            let _ = audio.pause();
        }
        self.sync_state();
    }

    fn stop(&mut self) {
        if let Some(audio) = self.audio.as_ref() {
            let _ = audio.pause();
            audio.set_current_time(0.0);
        }
        self.sync_state();
    }

    fn seek(&mut self, position: f64) {
        if let Some(audio) = self.audio.as_ref() {
            audio.set_current_time(position.max(0.0));
        }
        self.sync_state();
    }

    fn seek_and_play(&mut self, position: f64) {
        if let Some(audio) = self.audio.as_ref() {
            audio.set_current_time(position.max(0.0));
            let _ = audio.play();
        }
        self.sync_state();
    }

    fn set_speed(&mut self, speed: f64) {
        if let Some(audio) = self.audio.as_ref() {
            audio.set_playback_rate(speed.clamp(0.5, 2.0));
        }
        self.sync_state();
    }

    fn set_volume(&mut self, volume: f64) {
        if let Some(audio) = self.audio.as_ref() {
            audio.set_volume(volume.clamp(0.0, 1.0));
        }
        self.sync_state();
    }

    fn set_balance(&mut self, _balance: f64) {}

    fn set_loop(&mut self, loop_enabled: bool) {
        if let Some(audio) = self.audio.as_ref() {
            audio.set_loop(loop_enabled);
        }
    }
}
