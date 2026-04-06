use choreo_master_mobile_json::Color;
use choreo_master_mobile_json::SceneId;
use choreo_models::ChoreographyModel;

pub use crate::scene_list_item::SceneItemState;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ScenesState {
    pub choreography: ChoreographyModel,
    pub scenes: Vec<SceneItemState>,
    pub visible_scenes: Vec<SceneItemState>,
    pub selected_scene: Option<SceneItemState>,
    pub search_text: String,
    pub show_timestamps: bool,
    pub is_place_mode: bool,
    pub can_save_choreo: bool,
    pub can_delete_scene: bool,
    pub can_navigate_to_settings: bool,
    pub can_navigate_to_dancer_settings: bool,
    pub show_delete_scene_dialog: bool,
    pub show_copy_scene_positions_dialog: bool,
    pub copy_scene_positions_decision: Option<bool>,
    pub request_open_choreo_dialog: bool,
    pub request_save_choreo: bool,
    pub navigate_to_settings_requested: bool,
    pub navigate_to_dancer_settings_requested: bool,
    pub last_opened_choreo_file: Option<String>,
    pub pending_open_audio: Option<String>,
    pub close_audio_requested: bool,
    pub delete_scene_requested: bool,
    pub delete_scene_requested_scene_id: Option<SceneId>,
    pub reload_requested: bool,
    pub redraw_floor_requested: bool,
    pub selected_scene_changed: bool,
    pub has_selected_scene: bool,
    pub selected_scene_name: String,
    pub selected_scene_text: String,
    pub selected_scene_fixed_positions: bool,
    pub selected_scene_timestamp_text: String,
    pub selected_scene_color: Color,
    pub delete_scene_dialog_scene: Option<SceneItemState>,
}

impl ScenesState {
    pub fn clear_ephemeral_outputs(&mut self) {
        self.pending_open_audio = None;
        self.close_audio_requested = false;
        self.delete_scene_requested = false;
        self.delete_scene_requested_scene_id = None;
        self.reload_requested = false;
        self.redraw_floor_requested = false;
        self.selected_scene_changed = false;
        self.copy_scene_positions_decision = None;
        self.request_open_choreo_dialog = false;
        self.request_save_choreo = false;
        self.navigate_to_settings_requested = false;
        self.navigate_to_dancer_settings_requested = false;
    }
}

pub fn parse_timestamp_seconds(value: &str) -> Option<f64> {
    let value = value.trim();
    if value.is_empty() {
        return None;
    }

    let mut parts = value.split(':').collect::<Vec<_>>();
    if parts.len() > 3 {
        return None;
    }

    let seconds_part = parts.pop()?;
    let minutes_part = parts.pop().unwrap_or("0");
    let hours_part = parts.pop().unwrap_or("0");

    let seconds = seconds_part.parse::<f64>().ok()?;
    let minutes = minutes_part.parse::<f64>().ok()?;
    let hours = hours_part.parse::<f64>().ok()?;

    Some(hours * 3600.0 + minutes * 60.0 + seconds)
}

pub fn format_seconds(value: f64) -> String {
    let mut text = format!("{value:.3}");
    if text.contains('.') {
        while text.ends_with('0') {
            text.pop();
        }
        if text.ends_with('.') {
            text.pop();
        }
        if text.is_empty() {
            text.push('0');
        }
    }
    text
}

pub fn normalize_text(value: &str) -> Option<String> {
    if value.trim().is_empty() {
        None
    } else {
        Some(value.trim().to_string())
    }
}

pub fn build_scene_name(scenes: &[SceneItemState]) -> String {
    const BASE_NAME: &str = "New Scene";
    if scenes.iter().all(|scene| scene.name != BASE_NAME) {
        return BASE_NAME.to_string();
    }

    let mut suffix = 2;
    loop {
        let candidate = format!("{BASE_NAME} {suffix}");
        if scenes.iter().all(|scene| scene.name != candidate) {
            return candidate;
        }
        suffix += 1;
    }
}

pub fn next_scene_id(scenes: &[SceneItemState]) -> SceneId {
    let mut next: i64 = 0;
    for scene in scenes {
        next = next.max(scene.scene_id.0 as i64);
    }
    SceneId(next.saturating_add(1) as i32)
}
