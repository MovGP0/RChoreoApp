#[derive(Debug, Clone, PartialEq)]
pub struct AudioPlayerScene {
    pub scene_id: i32,
    pub name: String,
    pub timestamp: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AudioPlayerChoreographyScene {
    pub scene_id: i32,
    pub timestamp: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AudioPlayerState {
    pub speed: f64,
    pub minimum_speed: f64,
    pub maximum_speed: f64,
    pub volume: f64,
    pub balance: f64,
    pub duration: f64,
    pub position: f64,
    pub tick_values: Vec<f64>,
    pub speed_label: String,
    pub duration_label: String,
    pub can_link_scene_to_position: bool,
    pub is_playing: bool,
    pub loop_enabled: bool,
    pub can_seek: bool,
    pub can_set_speed: bool,
    pub has_stream_factory: bool,
    pub has_player: bool,
    pub preparation_seconds: f64,
    pub pause_seconds: f64,
    pub is_user_dragging: bool,
    pub was_playing_before_drag: bool,
    pub is_adjusting_speed: bool,
    pub pending_seek_position: Option<f64>,
    pub last_opened_audio_file_path: Option<String>,
    pub scenes: Vec<AudioPlayerScene>,
    pub selected_scene_id: Option<i32>,
    pub choreography_scenes: Vec<AudioPlayerChoreographyScene>,
    pub last_published_position: Option<f64>,
}

impl Default for AudioPlayerState {
    fn default() -> Self {
        Self {
            speed: 1.0,
            minimum_speed: 0.8,
            maximum_speed: 1.1,
            volume: 1.0,
            balance: 0.0,
            duration: 0.0,
            position: 0.0,
            tick_values: Vec::new(),
            speed_label: speed_to_percent_text(1.0),
            duration_label: duration_label(0.0, 0.0),
            can_link_scene_to_position: false,
            is_playing: false,
            loop_enabled: false,
            can_seek: false,
            can_set_speed: false,
            has_stream_factory: false,
            has_player: false,
            preparation_seconds: 4.0,
            pause_seconds: 0.0,
            is_user_dragging: false,
            was_playing_before_drag: false,
            is_adjusting_speed: false,
            pending_seek_position: None,
            last_opened_audio_file_path: None,
            scenes: Vec::new(),
            selected_scene_id: None,
            choreography_scenes: Vec::new(),
            last_published_position: None,
        }
    }
}

pub enum PlayPauseGlyph {
    Play,
    Pause,
}

#[must_use]
pub fn play_pause_glyph(is_playing: bool) -> PlayPauseGlyph {
    if is_playing {
        PlayPauseGlyph::Pause
    } else {
        PlayPauseGlyph::Play
    }
}

#[must_use]
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

#[must_use]
pub fn speed_to_percent_text(speed: f64) -> String {
    if !speed.is_finite() {
        return "0%".to_string();
    }

    let percent = (speed * 100.0).round() as i64;
    format!("{percent}%")
}

#[must_use]
pub fn duration_label(position: f64, duration: f64) -> String {
    format!(
        "{} / {}",
        duration_to_time_text(position),
        duration_to_time_text(duration),
    )
}
