use choreo_models::SceneModel;

use super::actions::ChoreographySettingsAction;
use super::actions::UpdateSelectedSceneAction;
use super::state::ChoreographySettingsState;
use super::state::current_date_parts;
use super::state::DateParts;
use time::Date;
use time::Month;

pub fn reduce(state: &mut ChoreographySettingsState, action: ChoreographySettingsAction) {
    match action {
        ChoreographySettingsAction::LoadChoreography {
            choreography,
            selected_scene,
        } => {
            state.choreography = *choreography;
            state.selected_scene = selected_scene;
            if state.choreography.name.is_empty()
                && state.choreography.scenes.is_empty()
                && state.choreography.comment.is_none()
            {
                reset_state_from_empty_choreography(state);
            } else {
                map_state_from_choreography(state);
                sync_state_from_selected_scene(state);
            }
        }
        ChoreographySettingsAction::LoadSettingsPreferences {
            show_timestamps,
            positions_at_side,
            snap_to_grid,
        } => {
            state.show_timestamps = show_timestamps;
            state.positions_at_side = positions_at_side;
            state.snap_to_grid = snap_to_grid;
            state.preferences.show_timestamps = show_timestamps;
            state.preferences.positions_at_side = positions_at_side;
            state.preferences.snap_to_grid = snap_to_grid;
            state.choreography.settings.show_timestamps = show_timestamps;
            state.choreography.settings.positions_at_side = positions_at_side;
            state.choreography.settings.snap_to_grid = snap_to_grid;
        }
        ChoreographySettingsAction::InitializeDrawPathFrom(value) => {
            state.preferences.draw_path_from = value;
            state.draw_path_from = value;
        }
        ChoreographySettingsAction::InitializeDrawPathTo(value) => {
            state.preferences.draw_path_to = value;
            state.draw_path_to = value;
        }
        ChoreographySettingsAction::InitializeShowLegend(value) => {
            state.preferences.show_legend = value;
            state.show_legend = value;
        }
        ChoreographySettingsAction::InitializeShowTimestamps(value) => {
            state.preferences.show_timestamps = value;
            state.show_timestamps = value;
            state.choreography.settings.show_timestamps = value;
        }
        ChoreographySettingsAction::InitializePositionsAtSide(value) => {
            state.preferences.positions_at_side = value;
            state.positions_at_side = value;
            state.choreography.settings.positions_at_side = value;
        }
        ChoreographySettingsAction::InitializeSnapToGrid(value) => {
            state.preferences.snap_to_grid = value;
            state.snap_to_grid = value;
            state.choreography.settings.snap_to_grid = value;
        }
        ChoreographySettingsAction::UpdateComment(value) => {
            state.choreography.comment = normalize_text(&value);
            state.comment = state.choreography.comment.clone().unwrap_or_default();
            state.redraw_requested = true;
        }
        ChoreographySettingsAction::UpdateName(value) => {
            state.choreography.name = value.trim().to_string();
            state.name = state.choreography.name.clone();
            state.redraw_requested = true;
        }
        ChoreographySettingsAction::UpdateSubtitle(value) => {
            state.choreography.subtitle = normalize_text(&value);
            state.subtitle = state.choreography.subtitle.clone().unwrap_or_default();
            state.redraw_requested = true;
        }
        ChoreographySettingsAction::UpdateDate { year, month, day } => {
            state.date = DateParts { year, month, day };
            state.choreography.date = build_date(year, month, day);
            state.redraw_requested = true;
        }
        ChoreographySettingsAction::UpdateVariation(value) => {
            state.choreography.variation = normalize_text(&value);
            state.variation = state.choreography.variation.clone().unwrap_or_default();
            state.redraw_requested = true;
        }
        ChoreographySettingsAction::UpdateAuthor(value) => {
            state.choreography.author = normalize_text(&value);
            state.author = state.choreography.author.clone().unwrap_or_default();
            state.redraw_requested = true;
        }
        ChoreographySettingsAction::UpdateDescription(value) => {
            state.choreography.description = normalize_text(&value);
            state.description = state.choreography.description.clone().unwrap_or_default();
            state.redraw_requested = true;
        }
        ChoreographySettingsAction::UpdateFloorFront(value) => {
            state.floor_front = value.clamp(1, 100);
            state.choreography.floor.size_front = state.floor_front;
            state.redraw_requested = true;
        }
        ChoreographySettingsAction::UpdateFloorBack(value) => {
            state.floor_back = value.clamp(1, 100);
            state.choreography.floor.size_back = state.floor_back;
            state.redraw_requested = true;
        }
        ChoreographySettingsAction::UpdateFloorLeft(value) => {
            state.floor_left = value.clamp(1, 100);
            state.choreography.floor.size_left = state.floor_left;
            state.redraw_requested = true;
        }
        ChoreographySettingsAction::UpdateFloorRight(value) => {
            state.floor_right = value.clamp(1, 100);
            state.choreography.floor.size_right = state.floor_right;
            state.redraw_requested = true;
        }
        ChoreographySettingsAction::UpdateGridResolution(value) => {
            let clamped = value.clamp(1, 16);
            state.set_grid_resolution(clamped);
            state.choreography.settings.resolution = clamped;
            state.redraw_requested = true;
        }
        ChoreographySettingsAction::UpdateDrawPathFrom(value) => {
            state.draw_path_from = value;
            state.preferences.draw_path_from = value;
            state.redraw_requested = true;
        }
        ChoreographySettingsAction::UpdateDrawPathTo(value) => {
            state.draw_path_to = value;
            state.preferences.draw_path_to = value;
            state.redraw_requested = true;
        }
        ChoreographySettingsAction::UpdateGridLines(value) => {
            state.grid_lines = value;
            state.choreography.settings.grid_lines = value;
            state.redraw_requested = true;
        }
        ChoreographySettingsAction::UpdateSnapToGrid(value) => {
            state.snap_to_grid = value;
            state.preferences.snap_to_grid = value;
            state.choreography.settings.snap_to_grid = value;
            state.redraw_requested = true;
        }
        ChoreographySettingsAction::UpdateShowTimestamps(value) => {
            state.show_timestamps = value;
            state.preferences.show_timestamps = value;
            state.choreography.settings.show_timestamps = value;
            state.redraw_requested = true;
            state.last_show_timestamps_event = Some(super::state::ShowTimestampsChangedEvent {
                is_enabled: value,
            });
        }
        ChoreographySettingsAction::UpdateShowLegend(value) => {
            state.show_legend = value;
            state.preferences.show_legend = value;
            state.redraw_requested = true;
        }
        ChoreographySettingsAction::UpdatePositionsAtSide(value) => {
            state.positions_at_side = value;
            state.preferences.positions_at_side = value;
            state.choreography.settings.positions_at_side = value;
            state.redraw_requested = true;
        }
        ChoreographySettingsAction::UpdateTransparency(value) => {
            state.transparency = value.clamp(0.0, 1.0);
            state.choreography.settings.transparency = state.transparency;
            state.redraw_requested = true;
        }
        ChoreographySettingsAction::UpdateFloorColor(value) => {
            state.floor_color = value.clone();
            state.choreography.settings.floor_color = value;
            state.redraw_requested = true;
        }
        ChoreographySettingsAction::UpdateSelectedScene(action) => {
            reduce_selected_scene(state, action);
        }
        ChoreographySettingsAction::ClearEphemeralOutputs => {
            state.clear_ephemeral_outputs();
        }
    }
}

fn reduce_selected_scene(state: &mut ChoreographySettingsState, action: UpdateSelectedSceneAction) {
    match action {
        UpdateSelectedSceneAction::SyncFromSelected => {
            sync_state_from_selected_scene(state);
        }
        UpdateSelectedSceneAction::SceneName(value) => {
            if let Some(selected_scene) = state.selected_scene.as_mut() {
                let scene_id = selected_scene.scene_id;
                selected_scene.name = value.clone();
                if let Some(model_scene) = find_scene_mut(&mut state.choreography.scenes, scene_id) {
                    model_scene.name = value.clone();
                }
                state.scene_name = value;
            }
        }
        UpdateSelectedSceneAction::SceneText(value) => {
            if let Some(selected_scene) = state.selected_scene.as_mut() {
                let scene_id = selected_scene.scene_id;
                selected_scene.text = value.clone();
                if let Some(model_scene) = find_scene_mut(&mut state.choreography.scenes, scene_id) {
                    model_scene.text = normalize_text(&value);
                }
                state.scene_text = value;
            }
        }
        UpdateSelectedSceneAction::SceneFixedPositions(value) => {
            if let Some(selected_scene) = state.selected_scene.as_mut() {
                let scene_id = selected_scene.scene_id;
                selected_scene.fixed_positions = value;
                if let Some(model_scene) = find_scene_mut(&mut state.choreography.scenes, scene_id) {
                    model_scene.fixed_positions = value;
                }
                state.scene_fixed_positions = value;
            }
        }
        UpdateSelectedSceneAction::SceneColor(value) => {
            if let Some(selected_scene) = state.selected_scene.as_mut() {
                let scene_id = selected_scene.scene_id;
                selected_scene.color = value.clone();
                if let Some(model_scene) = find_scene_mut(&mut state.choreography.scenes, scene_id) {
                    model_scene.color = value.clone();
                }
                state.scene_color = value;
            }
        }
        UpdateSelectedSceneAction::SceneTimestamp {
            has_timestamp,
            seconds,
        } => {
            if let Some(selected_scene) = state.selected_scene.as_mut() {
                let scene_id = selected_scene.scene_id;
                selected_scene.timestamp = if has_timestamp { Some(seconds) } else { None };
                if let Some(model_scene) = find_scene_mut(&mut state.choreography.scenes, scene_id) {
                    model_scene.timestamp = if has_timestamp {
                        Some(format_seconds(seconds))
                    } else {
                        None
                    };
                }
                state.scene_has_timestamp = has_timestamp;
                state.set_scene_timestamp_seconds(seconds);
            }
        }
    }
}

fn reset_state_from_empty_choreography(state: &mut ChoreographySettingsState) {
    state.comment.clear();
    state.name.clear();
    state.subtitle.clear();
    state.variation.clear();
    state.author.clear();
    state.description.clear();
    state.date = current_date_parts();
    state.floor_front = 0;
    state.floor_back = 0;
    state.floor_left = 0;
    state.floor_right = 0;
    state.set_grid_resolution(1);
    state.transparency = 0.0;
    state.positions_at_side = false;
    state.grid_lines = false;
    state.snap_to_grid = true;
    state.floor_color = choreo_master_mobile_json::Color::transparent();
    state.show_timestamps = false;
    state.has_selected_scene = false;
    state.scene_name.clear();
    state.scene_text.clear();
    state.scene_fixed_positions = false;
    state.scene_has_timestamp = false;
    state.scene_timestamp_seconds = 0.0;
    state.scene_timestamp_minutes = 0;
    state.scene_timestamp_seconds_part = 0;
    state.scene_timestamp_millis = 0;
    state.scene_color = choreo_master_mobile_json::Color::transparent();
}

fn map_state_from_choreography(state: &mut ChoreographySettingsState) {
    state.comment = state.choreography.comment.clone().unwrap_or_default();
    state.name = state.choreography.name.clone();
    state.subtitle = state.choreography.subtitle.clone().unwrap_or_default();
    state.variation = state.choreography.variation.clone().unwrap_or_default();
    state.author = state.choreography.author.clone().unwrap_or_default();
    state.description = state.choreography.description.clone().unwrap_or_default();
    state.date = state
        .choreography
        .date
        .map(|value| DateParts {
            year: value.year(),
            month: value.month() as u8,
            day: value.day(),
        })
        .unwrap_or_else(current_date_parts);
    state.floor_front = state.choreography.floor.size_front;
    state.floor_back = state.choreography.floor.size_back;
    state.floor_left = state.choreography.floor.size_left;
    state.floor_right = state.choreography.floor.size_right;
    state.set_grid_resolution(state.choreography.settings.resolution);
    state.transparency = state.choreography.settings.transparency;
    state.positions_at_side = state.choreography.settings.positions_at_side;
    state.grid_lines = state.choreography.settings.grid_lines;
    state.snap_to_grid = state.choreography.settings.snap_to_grid;
    state.floor_color = state.choreography.settings.floor_color.clone();
    state.show_timestamps = state.choreography.settings.show_timestamps;
}

fn sync_state_from_selected_scene(state: &mut ChoreographySettingsState) {
    let Some(selected_scene) = state.selected_scene.as_ref() else {
        clear_selected_scene_state(state);
        return;
    };
    let scene_snapshot = state
        .choreography
        .scenes
        .iter()
        .find(|scene| scene.scene_id == selected_scene.scene_id)
        .map(|scene| {
            (
                scene.name.clone(),
                scene.text.clone().unwrap_or_default(),
                scene.fixed_positions,
                scene.timestamp.clone(),
                scene.color.clone(),
            )
        });

    state.has_selected_scene = scene_snapshot.is_some();
    if let Some((name, text, fixed_positions, timestamp, color)) = scene_snapshot {
        state.scene_name = name.clone();
        state.scene_text = text.clone();
        state.scene_fixed_positions = fixed_positions;
        state.scene_has_timestamp = timestamp.is_some();
        let seconds = timestamp
            .as_deref()
            .and_then(parse_timestamp_seconds)
            .unwrap_or(0.0);
        state.set_scene_timestamp_seconds(seconds);
        state.scene_color = color.clone();
        if let Some(selected_scene) = state.selected_scene.as_mut() {
            selected_scene.name = name;
            selected_scene.text = text;
            selected_scene.fixed_positions = fixed_positions;
            selected_scene.timestamp = if state.scene_has_timestamp {
                Some(seconds)
            } else {
                None
            };
            selected_scene.color = color;
        }
    } else {
        clear_selected_scene_state(state);
    }
}

fn clear_selected_scene_state(state: &mut ChoreographySettingsState) {
    state.has_selected_scene = false;
    state.scene_name.clear();
    state.scene_text.clear();
    state.scene_fixed_positions = false;
    state.scene_has_timestamp = false;
    state.scene_timestamp_seconds = 0.0;
    state.scene_timestamp_minutes = 0;
    state.scene_timestamp_seconds_part = 0;
    state.scene_timestamp_millis = 0;
    state.scene_color = choreo_master_mobile_json::Color::transparent();
}

fn normalize_text(value: &str) -> Option<String> {
    if value.trim().is_empty() {
        None
    } else {
        Some(value.trim().to_string())
    }
}

fn find_scene_mut(
    scenes: &mut [SceneModel],
    scene_id: choreo_master_mobile_json::SceneId,
) -> Option<&mut SceneModel> {
    scenes.iter_mut().find(|scene| scene.scene_id == scene_id)
}

fn parse_timestamp_seconds(value: &str) -> Option<f64> {
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

fn format_seconds(value: f64) -> String {
    let mut text = format!("{value:.3}");
    if text.find('.').is_some() {
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

fn build_date(year: i32, month: u8, day: u8) -> Option<Date> {
    let month = month_from_u8(month)?;
    Date::from_calendar_date(year, month, day).ok()
}

fn month_from_u8(month: u8) -> Option<Month> {
    match month {
        1 => Some(Month::January),
        2 => Some(Month::February),
        3 => Some(Month::March),
        4 => Some(Month::April),
        5 => Some(Month::May),
        6 => Some(Month::June),
        7 => Some(Month::July),
        8 => Some(Month::August),
        9 => Some(Month::September),
        10 => Some(Month::October),
        11 => Some(Month::November),
        12 => Some(Month::December),
        _ => None,
    }
}
