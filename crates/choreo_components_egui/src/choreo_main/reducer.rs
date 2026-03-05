use super::actions::ChoreoMainAction;
use super::state::ChoreoMainState;
use super::state::InteractionMode;
use super::state::InteractionStateMachineState;
use super::state::MainContent;
use crate::audio_player::actions::AudioPlayerAction;
use crate::audio_player::reducer::AudioPlayerEffect;
use crate::choreography_settings::actions::ChoreographySettingsAction;
use crate::choreography_settings::actions::UpdateSelectedSceneAction;
use crate::choreography_settings::state::SelectedSceneState;
use crate::settings::actions::SettingsAction;
use choreo_master_mobile_json::Color;
use choreo_master_mobile_json::SceneId;
use choreo_models::SceneModel;

pub fn reduce(state: &mut ChoreoMainState, action: ChoreoMainAction) {
    match action {
        ChoreoMainAction::Initialize => {
            state.content = MainContent::Main;
            state.is_nav_open = false;
            state.is_choreography_settings_open = false;
            state.is_audio_player_open = false;
            sync_choreography_settings_projection(state);
        }
        ChoreoMainAction::ToggleNav => {
            state.is_nav_open = !state.is_nav_open;
        }
        ChoreoMainAction::CloseNav => {
            state.is_nav_open = false;
        }
        ChoreoMainAction::OpenSettings => {
            state.is_choreography_settings_open = true;
        }
        ChoreoMainAction::CloseSettings => {
            state.is_choreography_settings_open = false;
        }
        ChoreoMainAction::OpenAudioPanel => {
            state.is_audio_player_open = true;
        }
        ChoreoMainAction::CloseAudioPanel => {
            state.is_audio_player_open = false;
        }
        ChoreoMainAction::SelectMode { index } => {
            state.selected_mode_index = index;
            if let Some(mode) = interaction_mode_from_index(index) {
                state.interaction_mode = mode;
            }
        }
        ChoreoMainAction::ResetFloorViewport => {
            state.draw_floor_request_count += 1;
        }
        ChoreoMainAction::NavigateToSettings => {
            state.content = MainContent::Settings;
        }
        ChoreoMainAction::NavigateToMain => {
            state.content = MainContent::Main;
        }
        ChoreoMainAction::NavigateToDancers => {
            state.content = MainContent::Dancers;
        }
        ChoreoMainAction::ShowDialog { content } => {
            state.dialog_content = content;
            state.is_dialog_open = state.dialog_content.is_some();
        }
        ChoreoMainAction::HideDialog => {
            state.dialog_content = None;
            state.is_dialog_open = false;
        }
        ChoreoMainAction::RequestOpenChoreo(request) => {
            state.outgoing_open_choreo_requests.push(request);
        }
        ChoreoMainAction::RequestOpenAudio(request) => {
            state.outgoing_audio_requests.push(request);
        }
        ChoreoMainAction::RequestOpenImage { file_path } => {
            state
                .outgoing_open_svg_commands
                .push(super::actions::OpenSvgFileCommand { file_path });
        }
        ChoreoMainAction::ApplyOpenSvgFile(command) => {
            let path = command.file_path.trim();
            if path.is_empty() {
                state.svg_file_path = None;
                state.last_opened_svg_preference = None;
            } else {
                let normalized = path.to_string();
                state.svg_file_path = Some(normalized.clone());
                state.last_opened_svg_preference = Some(normalized);
            }
            state.draw_floor_request_count += 1;
        }
        ChoreoMainAction::RestoreLastOpenedSvg {
            file_path,
            path_exists,
        } => {
            let Some(path) = file_path.map(|value| value.trim().to_string()) else {
                return;
            };
            if path.is_empty() {
                return;
            }

            if !path_exists {
                state.last_opened_svg_preference = None;
                return;
            }

            state.svg_file_path = Some(path.clone());
            state.last_opened_svg_preference = Some(path);
            state.draw_floor_request_count += 1;
        }
        ChoreoMainAction::ApplyInteractionMode {
            mode,
            selected_positions_count,
        } => {
            state.interaction_mode = mode;
            state.selected_mode_index = interaction_mode_index(mode);
            state.selected_positions_count = selected_positions_count;
            state.interaction_state_machine = map_interaction_state(mode, selected_positions_count);
        }
        ChoreoMainAction::SetScenes { scenes } => {
            state.scenes = scenes;
            if !state.scenes.is_empty() {
                select_scene_internal(state, 0, false);
            }
            sync_choreography_settings_projection(state);
        }
        ChoreoMainAction::SelectScene { index } => {
            select_scene_internal(state, index, false);
            sync_choreography_settings_projection(state);
        }
        ChoreoMainAction::UpdateAudioPosition { seconds } => {
            sync_audio_position_internal(state, seconds);
        }
        ChoreoMainAction::LinkSelectedSceneToAudioPosition => {
            let Some(selected_index) = state.selected_scene_index else {
                return;
            };
            let Some(selected_scene) = state.scenes.get_mut(selected_index) else {
                return;
            };
            let linked_timestamp = round_to_100_millis(state.audio_position_seconds);
            selected_scene.timestamp_seconds = Some(linked_timestamp);
            state.audio_position_seconds = linked_timestamp;
            state.floor_scene_name = Some(selected_scene.name.clone());
            sync_choreography_settings_projection(state);
        }
        ChoreoMainAction::FloorAction(action) => {
            crate::floor::reducer::reduce(&mut state.floor_state, action);
        }
        ChoreoMainAction::AudioPlayerAction(action) => {
            let seek_position = match &action {
                AudioPlayerAction::SeekToPosition { position }
                | AudioPlayerAction::PositionDragCompleted { position } => Some(*position),
                _ => None,
            };

            let effects =
                crate::audio_player::reducer::reduce(&mut state.audio_player_state, action);

            if let Some(position) = seek_position {
                sync_audio_position_internal(state, position);
            }

            for effect in effects {
                match effect {
                    AudioPlayerEffect::PositionChangedPublished { position_seconds } => {
                        sync_audio_position_internal(state, position_seconds);
                    }
                }
            }
        }
        ChoreoMainAction::ChoreographySettingsAction(action) => {
            crate::choreography_settings::reducer::reduce(
                &mut state.choreography_settings_state,
                action,
            );
            sync_main_state_from_choreography_settings(state);
        }
        ChoreoMainAction::SettingsAction(action) => {
            crate::settings::reducer::reduce(&mut state.settings_state, action.clone());
            if matches!(action, SettingsAction::NavigateBack) {
                state.content = MainContent::Main;
            }
        }
        ChoreoMainAction::DancersAction(action) => {
            crate::dancers::reducer::reduce(&mut state.dancers_state, action);
        }
        ChoreoMainAction::ClearOutgoingCommands => {
            state.outgoing_open_choreo_requests.clear();
            state.outgoing_audio_requests.clear();
            state.outgoing_open_svg_commands.clear();
        }
    }
}

fn interaction_mode_from_index(index: i32) -> Option<InteractionMode> {
    match index {
        0 => Some(InteractionMode::View),
        1 => Some(InteractionMode::Move),
        2 => Some(InteractionMode::RotateAroundCenter),
        3 => Some(InteractionMode::RotateAroundDancer),
        4 => Some(InteractionMode::Scale),
        5 => Some(InteractionMode::LineOfSight),
        _ => None,
    }
}

fn interaction_mode_index(mode: InteractionMode) -> i32 {
    match mode {
        InteractionMode::View => 0,
        InteractionMode::Move => 1,
        InteractionMode::RotateAroundCenter => 2,
        InteractionMode::RotateAroundDancer => 3,
        InteractionMode::Scale => 4,
        InteractionMode::LineOfSight => 5,
    }
}

fn map_interaction_state(
    mode: InteractionMode,
    selected_positions_count: usize,
) -> InteractionStateMachineState {
    match mode {
        InteractionMode::Move => InteractionStateMachineState::MovePositions,
        InteractionMode::RotateAroundCenter if selected_positions_count == 0 => {
            InteractionStateMachineState::RotateAroundCenter
        }
        InteractionMode::RotateAroundCenter => {
            InteractionStateMachineState::RotateAroundCenterSelection
        }
        InteractionMode::RotateAroundDancer if selected_positions_count == 0 => {
            InteractionStateMachineState::ScaleAroundDancer
        }
        InteractionMode::RotateAroundDancer => {
            InteractionStateMachineState::ScaleAroundDancerSelection
        }
        InteractionMode::Scale if selected_positions_count == 0 => {
            InteractionStateMachineState::ScalePositions
        }
        InteractionMode::Scale => InteractionStateMachineState::ScalePositionsSelection,
        InteractionMode::View | InteractionMode::LineOfSight => InteractionStateMachineState::Idle,
    }
}

fn select_scene_internal(state: &mut ChoreoMainState, index: usize, keep_audio_position: bool) {
    let Some(scene) = state.scenes.get(index) else {
        return;
    };
    state.selected_scene_index = Some(index);
    state.floor_scene_name = Some(scene.name.clone());
    if !keep_audio_position && let Some(timestamp) = scene.timestamp_seconds {
        state.audio_position_seconds = timestamp;
    }
}

fn sync_audio_position_internal(state: &mut ChoreoMainState, seconds: f64) {
    state.audio_position_seconds = seconds;
    let target_scene = state
        .scenes
        .iter()
        .enumerate()
        .filter_map(|(index, scene)| {
            scene
                .timestamp_seconds
                .and_then(|timestamp| (timestamp <= seconds).then_some((index, timestamp)))
        })
        .max_by(|(_, left), (_, right)| left.total_cmp(right))
        .map(|(index, _)| index);

    if let Some(index) = target_scene {
        select_scene_internal(state, index, true);
        sync_choreography_settings_projection(state);
    }
}

fn sync_choreography_settings_projection(state: &mut ChoreoMainState) {
    let existing_scenes = state
        .choreography_settings_state
        .choreography
        .scenes
        .clone();
    state.choreography_settings_state.choreography.scenes = state
        .scenes
        .iter()
        .enumerate()
        .map(|(index, scene)| map_scene_state_to_model(index, scene, &existing_scenes))
        .collect();
    state.choreography_settings_state.selected_scene = state
        .selected_scene_index
        .and_then(|index| make_selected_scene_state(index, &state.scenes, &existing_scenes));
    crate::choreography_settings::reducer::reduce(
        &mut state.choreography_settings_state,
        ChoreographySettingsAction::UpdateSelectedScene(
            UpdateSelectedSceneAction::SyncFromSelected,
        ),
    );
}

fn sync_main_state_from_choreography_settings(state: &mut ChoreoMainState) {
    state.scenes = state
        .choreography_settings_state
        .choreography
        .scenes
        .iter()
        .map(|scene| super::state::SceneState {
            name: scene.name.clone(),
            timestamp_seconds: parse_scene_timestamp(scene.timestamp.as_deref()),
        })
        .collect();

    state.selected_scene_index = state
        .choreography_settings_state
        .selected_scene
        .as_ref()
        .and_then(|selected_scene| {
            state
                .choreography_settings_state
                .choreography
                .scenes
                .iter()
                .position(|scene| scene.scene_id == selected_scene.scene_id)
        });

    state.floor_scene_name = state
        .selected_scene_index
        .and_then(|index| state.scenes.get(index))
        .map(|scene| scene.name.clone());

    state.floor_state.floor_front = state.choreography_settings_state.floor_front;
    state.floor_state.floor_back = state.choreography_settings_state.floor_back;
    state.floor_state.floor_left = state.choreography_settings_state.floor_left;
    state.floor_state.floor_right = state.choreography_settings_state.floor_right;
    state.floor_state.snap_to_grid = state.choreography_settings_state.snap_to_grid;
    state.floor_state.grid_resolution = state.choreography_settings_state.grid_resolution();
    if state.choreography_settings_state.redraw_requested {
        state.draw_floor_request_count += 1;
    }
}

fn map_scene_state_to_model(
    index: usize,
    scene: &super::state::SceneState,
    existing_scenes: &[SceneModel],
) -> SceneModel {
    let scene_id = synthetic_scene_id(index);
    let existing_scene = existing_scenes
        .iter()
        .find(|existing| existing.scene_id == scene_id);

    SceneModel {
        scene_id,
        positions: existing_scene
            .map(|value| value.positions.clone())
            .unwrap_or_default(),
        name: scene.name.clone(),
        text: existing_scene.and_then(|value| value.text.clone()),
        fixed_positions: existing_scene.is_some_and(|value| value.fixed_positions),
        timestamp: scene.timestamp_seconds.map(format_scene_timestamp),
        variation_depth: existing_scene
            .map(|value| value.variation_depth)
            .unwrap_or_default(),
        variations: existing_scene
            .map(|value| value.variations.clone())
            .unwrap_or_default(),
        current_variation: existing_scene
            .map(|value| value.current_variation.clone())
            .unwrap_or_default(),
        color: existing_scene
            .map(|value| value.color.clone())
            .unwrap_or_else(Color::transparent),
    }
}

fn make_selected_scene_state(
    index: usize,
    scenes: &[super::state::SceneState],
    existing_scenes: &[SceneModel],
) -> Option<SelectedSceneState> {
    let scene = scenes.get(index)?;
    let scene_id = synthetic_scene_id(index);
    let existing_scene = existing_scenes
        .iter()
        .find(|existing| existing.scene_id == scene_id);

    Some(SelectedSceneState {
        scene_id,
        name: scene.name.clone(),
        text: existing_scene
            .and_then(|value| value.text.clone())
            .unwrap_or_default(),
        fixed_positions: existing_scene.is_some_and(|value| value.fixed_positions),
        timestamp: scene.timestamp_seconds,
        color: existing_scene
            .map(|value| value.color.clone())
            .unwrap_or_else(Color::transparent),
    })
}

fn synthetic_scene_id(index: usize) -> SceneId {
    SceneId(index as i32 + 1)
}

fn parse_scene_timestamp(value: Option<&str>) -> Option<f64> {
    value.and_then(|text| text.parse::<f64>().ok())
}

fn format_scene_timestamp(value: f64) -> String {
    let mut text = format!("{value:.3}");
    while text.ends_with('0') {
        text.pop();
    }
    if text.ends_with('.') {
        text.pop();
    }
    if text.is_empty() {
        text.push('0');
    }
    text
}

fn round_to_100_millis(seconds: f64) -> f64 {
    let milliseconds = seconds * 1000.0;
    let rounded = (milliseconds / 100.0).round() * 100.0;
    rounded / 1000.0
}
