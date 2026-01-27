use choreo_master_mobile_json::{Color, SceneId};
use choreo_models::{ChoreographyModel, SceneModel};

use crate::date;
use crate::scenes::SceneViewModel;
use crate::time::parse_timestamp_seconds;

pub(crate) use crate::time::format_seconds;

use super::choreography_settings_view_model::ChoreographySettingsViewModel;

pub(crate) fn reset_view_model(view_model: &mut ChoreographySettingsViewModel) {
    view_model.comment.clear();
    view_model.name.clear();
    view_model.subtitle.clear();
    view_model.variation.clear();
    view_model.author.clear();
    view_model.description.clear();
    view_model.date = date::today_date();
    view_model.floor_front = 0;
    view_model.floor_back = 0;
    view_model.floor_left = 0;
    view_model.floor_right = 0;
    view_model.set_grid_resolution(1);
    view_model.transparency = 0.0;
    view_model.positions_at_side = false;
    view_model.grid_lines = false;
    view_model.snap_to_grid = true;
    view_model.floor_color = Color::transparent();
    view_model.show_timestamps = false;
    view_model.has_selected_scene = false;
    view_model.scene_name.clear();
    view_model.scene_text.clear();
    view_model.scene_fixed_positions = false;
    view_model.scene_has_timestamp = false;
    view_model.scene_timestamp_seconds = 0.0;
    view_model.scene_timestamp_minutes = 0;
    view_model.scene_timestamp_seconds_part = 0;
    view_model.scene_timestamp_millis = 0;
    view_model.scene_color = Color::transparent();
}

pub(crate) fn map_from_choreography(
    choreography: &ChoreographyModel,
    view_model: &mut ChoreographySettingsViewModel,
) {
    view_model.comment = choreography.comment.clone().unwrap_or_default();
    view_model.name = choreography.name.clone();
    view_model.subtitle = choreography.subtitle.clone().unwrap_or_default();
    view_model.variation = choreography.variation.clone().unwrap_or_default();
    view_model.author = choreography.author.clone().unwrap_or_default();
    view_model.description = choreography.description.clone().unwrap_or_default();
    view_model.date = choreography.date.unwrap_or_else(date::today_date);

    view_model.floor_front = choreography.floor.size_front;
    view_model.floor_back = choreography.floor.size_back;
    view_model.floor_left = choreography.floor.size_left;
    view_model.floor_right = choreography.floor.size_right;
    view_model.set_grid_resolution(choreography.settings.resolution);
    view_model.transparency = choreography.settings.transparency;
    view_model.positions_at_side = choreography.settings.positions_at_side;
    view_model.grid_lines = choreography.settings.grid_lines;
    view_model.snap_to_grid = choreography.settings.snap_to_grid;
    view_model.floor_color = choreography.settings.floor_color.clone();
    view_model.show_timestamps = choreography.settings.show_timestamps;
}

pub(crate) fn update_selected_scene(
    view_model: &mut ChoreographySettingsViewModel,
    selected_scene: &Option<SceneViewModel>,
    choreography: &ChoreographyModel,
) {
    let Some(selected_scene) = selected_scene else {
        view_model.has_selected_scene = false;
        view_model.scene_name.clear();
        view_model.scene_text.clear();
        view_model.scene_fixed_positions = false;
        view_model.scene_has_timestamp = false;
        view_model.scene_timestamp_seconds = 0.0;
        view_model.scene_timestamp_minutes = 0;
        view_model.scene_timestamp_seconds_part = 0;
        view_model.scene_timestamp_millis = 0;
        view_model.scene_color = Color::transparent();
        return;
    };

    let scene_model = choreography
        .scenes
        .iter()
        .find(|scene| scene.scene_id == selected_scene.scene_id);

    view_model.has_selected_scene = scene_model.is_some();
    if let Some(scene_model) = scene_model {
        view_model.scene_name = scene_model.name.clone();
        view_model.scene_text = scene_model.text.clone().unwrap_or_default();
        view_model.scene_fixed_positions = scene_model.fixed_positions;
        view_model.scene_has_timestamp = scene_model.timestamp.is_some();
        let seconds = scene_model
            .timestamp
            .as_deref()
            .and_then(parse_timestamp_seconds)
            .unwrap_or(0.0);
        view_model.set_scene_timestamp_seconds(seconds);
        view_model.scene_color = scene_model.color.clone();
    }
}

pub(crate) fn normalize_text(value: &str) -> Option<String> {
    if value.trim().is_empty() {
        None
    } else {
        Some(value.trim().to_string())
    }
}

pub(crate) fn find_scene_mut(
    choreography: &mut ChoreographyModel,
    scene_id: Option<SceneId>,
) -> Option<&mut SceneModel> {
    let scene_id = scene_id?;
    choreography
        .scenes
        .iter_mut()
        .find(|scene| scene.scene_id == scene_id)
}


