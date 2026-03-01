use choreo_models::SceneModel;

use super::actions::ScenesAction;
use super::state::SceneItemState;
use super::state::ScenesState;
use super::state::build_scene_name;
use super::state::format_seconds;
use super::state::next_scene_id;
use super::state::normalize_text;
use super::state::parse_timestamp_seconds;

pub fn reduce(state: &mut ScenesState, action: ScenesAction) {
    match action {
        ScenesAction::LoadScenes { choreography } => {
            state.choreography = *choreography;
            map_scenes_from_choreography(state);
            select_first_scene(state);
            state.show_timestamps = state.choreography.settings.show_timestamps;
            state.selected_scene_changed = state.selected_scene.is_some();
        }
        ScenesAction::ReloadScenes => {
            map_scenes_from_choreography(state);
            if let Some(selected_id) = state.selected_scene.as_ref().map(|scene| scene.scene_id) {
                set_selected_scene_by_id(state, selected_id);
            } else {
                select_first_scene(state);
            }
            state.reload_requested = true;
        }
        ScenesAction::UpdateSearchText(value) => {
            state.search_text = value;
            refresh_visible_scenes(state);
        }
        ScenesAction::InsertScene { insert_after } => {
            let insert_index = match state.selected_scene.as_ref().map(|scene| scene.scene_id) {
                None => state.scenes.len(),
                Some(selected_id) => state
                    .scenes
                    .iter()
                    .position(|scene| scene.scene_id == selected_id)
                    .map(|index| if insert_after { index + 1 } else { index })
                    .unwrap_or(state.scenes.len()),
            };

            let name = build_scene_name(&state.scenes);
            let scene_id = next_scene_id(&state.scenes);
            let new_scene = SceneItemState::new(
                scene_id,
                name.clone(),
                choreo_master_mobile_json::Color::transparent(),
            );
            state.scenes.insert(insert_index, new_scene.clone());
            state.choreography.scenes.insert(
                insert_index.min(state.choreography.scenes.len()),
                SceneModel {
                    scene_id,
                    positions: Vec::new(),
                    name,
                    text: None,
                    fixed_positions: false,
                    timestamp: None,
                    variation_depth: 0,
                    variations: Vec::new(),
                    current_variation: Vec::new(),
                    color: choreo_master_mobile_json::Color::transparent(),
                },
            );
            set_selected_scene_by_id(state, new_scene.scene_id);
            refresh_visible_scenes(state);
            update_can_save(state);
        }
        ScenesAction::SelectScene { index } => {
            let Some(scene_id) = state.visible_scenes.get(index).map(|scene| scene.scene_id) else {
                return;
            };
            set_selected_scene_by_id(state, scene_id);
            state.selected_scene_changed = true;
            state.redraw_floor_requested = true;
        }
        ScenesAction::SelectSceneFromAudioPosition { position_seconds } => {
            let previous_id = state.selected_scene.as_ref().map(|scene| scene.scene_id);
            if state.scenes.len() < 2 {
                return;
            }

            for index in 0..state.scenes.len() {
                let current = &state.scenes[index];
                let Some(current_timestamp) = current.timestamp else {
                    continue;
                };
                let current_scene_id = current.scene_id;
                let next_index = index + 1;
                if next_index >= state.scenes.len() {
                    break;
                }
                let next = &state.scenes[next_index];
                let Some(next_timestamp) = next.timestamp else {
                    continue;
                };
                if next_timestamp <= current_timestamp {
                    continue;
                }
                if position_seconds >= current_timestamp && position_seconds <= next_timestamp {
                    set_selected_scene_by_id(state, current_scene_id);
                    let changed = previous_id != Some(current_scene_id);
                    if changed {
                        state.selected_scene_changed = true;
                        state.redraw_floor_requested = true;
                    }
                    return;
                }
            }
        }
        ScenesAction::ApplyPlacementModeForSelected => {
            let Some(selected) = state.selected_scene.as_ref() else {
                state.is_place_mode = false;
                return;
            };
            let dancer_count = state.choreography.dancers.len();
            let position_count = state
                .choreography
                .scenes
                .iter()
                .find(|scene| scene.scene_id == selected.scene_id)
                .map(|scene| scene.positions.len())
                .unwrap_or_default();
            state.is_place_mode = dancer_count > 0 && position_count < dancer_count;
        }
        ScenesAction::SyncShowTimestampsFromChoreography => {
            state.show_timestamps = state.choreography.settings.show_timestamps;
        }
        ScenesAction::UpdateShowTimestamps(value) => {
            state.show_timestamps = value;
            state.choreography.settings.show_timestamps = value;
        }
        ScenesAction::OpenChoreography {
            choreography,
            file_path,
            file_name,
            audio_path,
        } => {
            state.choreography = *choreography;
            map_scenes_from_choreography(state);
            select_first_scene(state);
            state.reload_requested = true;
            state.show_timestamps = state.choreography.settings.show_timestamps;
            state.last_opened_choreo_file = file_path.or(file_name);
            state.pending_open_audio = audio_path;
            state.close_audio_requested = state.pending_open_audio.is_none();
            update_can_save(state);
        }
        ScenesAction::SaveChoreography => {
            state.choreography.scenes = state
                .scenes
                .iter()
                .map(map_scene_item_to_model)
                .collect::<Vec<_>>();
            update_can_save(state);
        }
        ScenesAction::ClearEphemeralOutputs => {
            state.clear_ephemeral_outputs();
        }
    }
}

fn map_scenes_from_choreography(state: &mut ScenesState) {
    state.scenes = state
        .choreography
        .scenes
        .iter()
        .map(map_model_to_scene_item)
        .collect::<Vec<_>>();
    refresh_visible_scenes(state);
}

fn refresh_visible_scenes(state: &mut ScenesState) {
    if state.search_text.trim().is_empty() {
        state.visible_scenes = state.scenes.clone();
        return;
    }
    let search = state.search_text.to_ascii_lowercase();
    state.visible_scenes = state
        .scenes
        .iter()
        .filter(|scene| scene.name.to_ascii_lowercase().contains(&search))
        .cloned()
        .collect::<Vec<_>>();
}

fn select_first_scene(state: &mut ScenesState) {
    let selected = state.scenes.first().map(|scene| scene.scene_id);
    if let Some(scene_id) = selected {
        set_selected_scene_by_id(state, scene_id);
    } else {
        state.selected_scene = None;
    }
}

fn set_selected_scene_by_id(state: &mut ScenesState, scene_id: choreo_master_mobile_json::SceneId) {
    for scene in &mut state.scenes {
        scene.is_selected = scene.scene_id == scene_id;
    }
    for scene in &mut state.visible_scenes {
        scene.is_selected = scene.scene_id == scene_id;
    }
    state.selected_scene = state.scenes.iter().find(|scene| scene.scene_id == scene_id).cloned();
}

fn update_can_save(state: &mut ScenesState) {
    let has_file = state
        .last_opened_choreo_file
        .as_ref()
        .is_some_and(|path| !path.trim().is_empty());
    let has_choreo = !state.choreography.name.is_empty() || !state.choreography.scenes.is_empty();
    state.can_save_choreo = has_file && has_choreo;
}

fn map_model_to_scene_item(source: &SceneModel) -> SceneItemState {
    let mut target = SceneItemState::new(source.scene_id, source.name.clone(), source.color.clone());
    target.text = source.text.clone().unwrap_or_default();
    target.fixed_positions = source.fixed_positions;
    target.timestamp = source
        .timestamp
        .as_deref()
        .and_then(parse_timestamp_seconds);
    target.positions = source.positions.clone();
    target.variation_depth = source.variation_depth;
    target.variations = source.variations.clone();
    target.current_variation = source.current_variation.clone();
    target
}

fn map_scene_item_to_model(source: &SceneItemState) -> SceneModel {
    SceneModel {
        scene_id: source.scene_id,
        positions: source.positions.clone(),
        name: source.name.clone(),
        text: normalize_text(&source.text),
        fixed_positions: source.fixed_positions,
        timestamp: source.timestamp.map(format_seconds),
        variation_depth: source.variation_depth,
        variations: source.variations.clone(),
        current_variation: source.current_variation.clone(),
        color: source.color.clone(),
    }
}
