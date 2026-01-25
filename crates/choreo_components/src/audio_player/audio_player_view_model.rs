use super::types::{AudioPlayer, HapticFeedback, StreamFactory};

pub struct AudioPlayerViewModel {
    pub speed: f64,
    pub minimum_speed: f64,
    pub maximum_speed: f64,
    pub volume: f64,
    pub balance: f64,
    pub duration: f64,
    pub position: f64,
    pub tick_values: String,
    pub can_link_scene_to_position: bool,
    pub is_playing: bool,
    pub loop_enabled: bool,
    pub can_seek: bool,
    pub can_set_speed: bool,
    pub stream_factory: Option<StreamFactory>,
    pub preparation_seconds: f64,
    pub pause_seconds: f64,
    pub title: String,
    pub player: Option<Box<dyn AudioPlayer>>,
    pub haptic_feedback: Option<Box<dyn HapticFeedback>>,
}

impl AudioPlayerViewModel {
    pub fn new(haptic_feedback: Option<Box<dyn HapticFeedback>>) -> Self {
        Self {
            speed: 1.0,
            minimum_speed: 0.8,
            maximum_speed: 1.1,
            volume: 1.0,
            balance: 0.0,
            duration: 0.0,
            position: 0.0,
            tick_values: String::new(),
            can_link_scene_to_position: false,
            is_playing: false,
            loop_enabled: false,
            can_seek: false,
            can_set_speed: false,
            stream_factory: None,
            preparation_seconds: 4.0,
            pause_seconds: 0.0,
            title: "Audio".to_string(),
            player: None,
            haptic_feedback,
        }
    }

    pub fn toggle_play_pause(&mut self) {
        if let Some(haptic) = &self.haptic_feedback
            && haptic.is_supported()
        {
            haptic.perform_click();
        }

        let Some(player) = self.player.as_mut() else {
            return;
        };

        if player.is_playing() {
            player.pause();
            self.is_playing = false;
            return;
        }

        player.play();
        self.is_playing = true;
    }

    pub fn stop(&mut self) {
        let Some(player) = self.player.as_mut() else {
            return;
        };

        player.stop();
        self.is_playing = false;
        self.position = 0.0;
    }

    pub fn seek(&mut self, position: f64) {
        let Some(player) = self.player.as_mut() else {
            return;
        };

        if !player.can_seek() {
            return;
        }

        player.seek(position);
        self.position = player.current_position();
    }

    pub fn reload(&mut self) {
        // handled by the stream factory integration
    }

    pub fn link_scene_to_position(&mut self) {
        if let Some(haptic) = &self.haptic_feedback
            && haptic.is_supported()
        {
            haptic.perform_click();
        }
        // handled by behavior integration
    }
}

pub struct AudioPlayerViewState {
    is_user_dragging: bool,
    was_playing: bool,
    is_adjusting_speed: bool,
}

impl Default for AudioPlayerViewState {
    fn default() -> Self {
        Self::new()
    }
}

impl AudioPlayerViewState {
    const SPEED_SNAP_STEP: f64 = 0.05;

    pub fn new() -> Self {
        Self {
            is_user_dragging: false,
            was_playing: false,
            is_adjusting_speed: false,
        }
    }

    pub fn on_position_drag_started(&mut self, view_model: &mut AudioPlayerViewModel) {
        self.is_user_dragging = true;
        self.was_playing = false;

        let Some(player) = view_model.player.as_mut() else {
            return;
        };

        if !player.is_playing() {
            return;
        }

        player.pause();
        view_model.is_playing = false;
        self.was_playing = true;
    }

    pub fn on_position_drag_completed(
        &mut self,
        view_model: &mut AudioPlayerViewModel,
        position: f64,
    ) {
        self.is_user_dragging = false;

        if !view_model.can_seek {
            return;
        }

        view_model.seek(position);

        if !self.was_playing {
            return;
        }

        let Some(player) = view_model.player.as_mut() else {
            return;
        };

        player.play();
        view_model.is_playing = true;
    }

    pub fn on_speed_changed(&mut self, view_model: &mut AudioPlayerViewModel, new_value: f64) {
        if self.is_adjusting_speed {
            return;
        }

        let snapped = snap_speed(
            new_value,
            view_model.minimum_speed,
            view_model.maximum_speed,
            Self::SPEED_SNAP_STEP,
        );

        if (snapped - new_value).abs() < 0.0001 {
            return;
        }

        self.is_adjusting_speed = true;
        view_model.speed = snapped;
        self.is_adjusting_speed = false;
    }

    pub fn is_user_dragging(&self) -> bool {
        self.is_user_dragging
    }
}

fn snap_speed(value: f64, min: f64, max: f64, step: f64) -> f64 {
    let clamped = value.clamp(min, max);
    let snapped = (clamped / step).round() * step;
    snapped.clamp(min, max)
}

pub enum PlayPauseGlyph {
    Play,
    Pause,
}

impl PlayPauseGlyph {
    pub fn as_icon_name(&self) -> &'static str {
        match self {
            PlayPauseGlyph::Play => "play",
            PlayPauseGlyph::Pause => "pause",
        }
    }
}

pub fn play_pause_glyph(is_playing: bool) -> PlayPauseGlyph {
    if is_playing {
        PlayPauseGlyph::Pause
    } else {
        PlayPauseGlyph::Play
    }
}

pub fn duration_to_time_text(seconds: f64) -> String {
    if !seconds.is_finite() || seconds < 0.0 {
        return "0:00".to_string();
    }

    let total_seconds = seconds.round() as i64;
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;

    if hours >= 1 {
        format!("{hours}:{minutes:02}:{seconds:02}")
    } else {
        format!("{minutes}:{seconds:02}")
    }
}
