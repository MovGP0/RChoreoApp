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
use crate::dancers::actions::DancersAction;
use crate::dancers::state as dancers_state;
use crate::floor::state::FloorPosition;
use crate::floor::state::SceneRenderPosition;
use crate::settings::actions::SettingsAction;
use choreo_master_mobile_json::Color;
use choreo_master_mobile_json::DancerId;
use choreo_master_mobile_json::SceneId;
use choreo_models::DancerModel;
use choreo_models::PositionModel;
use choreo_models::RoleModel;
use choreo_models::SceneModel;
use std::collections::HashMap;
use std::rc::Rc;

use super::open_audio_behavior;
use super::open_choreo_file_behavior;
use super::open_image_behavior;
use super::open_svg_file_behavior;

pub fn reduce(state: &mut ChoreoMainState, action: ChoreoMainAction) {
    reduce_with_behaviors(state, action);
}

pub fn reduce_with_behaviors(state: &mut ChoreoMainState, action: ChoreoMainAction) {
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
            sync_dancers_projection(state);
            crate::dancers::reducer::reduce(
                &mut state.dancers_state,
                DancersAction::LoadFromGlobal,
            );
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
            open_choreo_file_behavior::request_open_choreo(state, request);
        }
        ChoreoMainAction::RequestSaveChoreo => {
            if let Some(file_path) = state.last_opened_choreo_file.as_ref().filter(|path| {
                !path.trim().is_empty() && std::path::Path::new(path.as_str()).exists()
            }) {
                state
                    .outgoing_save_choreo_requests
                    .push(super::actions::SaveChoreoRequested {
                        file_path: file_path.clone(),
                    });
            }
        }
        ChoreoMainAction::RequestOpenAudio(request) => {
            open_audio_behavior::request_open_audio(state, request);
        }
        ChoreoMainAction::RequestOpenImage { file_path } => {
            open_image_behavior::request_open_image(state, file_path);
        }
        ChoreoMainAction::ApplyOpenSvgFile(command) => {
            open_svg_file_behavior::apply_open_svg_file(state, command);
        }
        ChoreoMainAction::RestoreLastOpenedSvg {
            file_path,
            path_exists,
        } => {
            open_svg_file_behavior::restore_last_opened_svg(state, file_path, path_exists);
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
        ChoreoMainAction::UpdateSceneSearchText(value) => {
            state.scene_search_text = value;
        }
        ChoreoMainAction::InsertScene { insert_after } => {
            insert_scene_internal(state, insert_after);
            sync_choreography_settings_projection(state);
        }
        ChoreoMainAction::DeleteSelectedScene => {
            delete_selected_scene_internal(state);
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
            if matches!(action, DancersAction::Cancel) {
                state.content = MainContent::Main;
                return;
            }
            let should_sync_choreography = matches!(&action, DancersAction::SaveToGlobal);
            crate::dancers::reducer::reduce(&mut state.dancers_state, action);
            if should_sync_choreography {
                sync_main_state_from_dancers(state);
            }
        }
        ChoreoMainAction::ClearOutgoingCommands => {
            state.outgoing_open_choreo_requests.clear();
            state.outgoing_save_choreo_requests.clear();
            state.outgoing_audio_requests.clear();
            state.outgoing_open_svg_commands.clear();
        }
    }
}

pub(crate) fn interaction_mode_from_index(index: i32) -> Option<InteractionMode> {
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

pub(crate) fn map_global_interaction_mode(mode: InteractionMode) -> crate::global::InteractionMode {
    match mode {
        InteractionMode::View => crate::global::InteractionMode::View,
        InteractionMode::Move => crate::global::InteractionMode::Move,
        InteractionMode::RotateAroundCenter => crate::global::InteractionMode::RotateAroundCenter,
        InteractionMode::RotateAroundDancer => crate::global::InteractionMode::RotateAroundDancer,
        InteractionMode::Scale => crate::global::InteractionMode::Scale,
        InteractionMode::LineOfSight => crate::global::InteractionMode::LineOfSight,
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

fn insert_scene_internal(state: &mut ChoreoMainState, insert_after: bool) {
    let insert_index = match state.selected_scene_index {
        Some(index) if insert_after => (index + 1).min(state.scenes.len()),
        Some(index) => index.min(state.scenes.len()),
        None => state.scenes.len(),
    };
    state.scenes.insert(
        insert_index,
        super::state::SceneState {
            name: build_new_scene_name(&state.scenes),
            timestamp_seconds: None,
        },
    );
    select_scene_internal(state, insert_index, false);
}

fn delete_selected_scene_internal(state: &mut ChoreoMainState) {
    let Some(index) = state.selected_scene_index else {
        return;
    };
    if index >= state.scenes.len() {
        return;
    }

    state.scenes.remove(index);
    if state.scenes.is_empty() {
        state.selected_scene_index = None;
        state.floor_scene_name = None;
        return;
    }

    let next_index = index.min(state.scenes.len() - 1);
    select_scene_internal(state, next_index, false);
}

pub(crate) fn sync_audio_position_internal(state: &mut ChoreoMainState, seconds: f64) {
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
        return;
    }

    refresh_floor_projection(state);
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
    state.scene_models = state
        .choreography_settings_state
        .choreography
        .scenes
        .clone();
    refresh_floor_projection(state);
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

    state.scene_models = state
        .choreography_settings_state
        .choreography
        .scenes
        .clone();
    state.floor_scene_name = state
        .selected_scene_index
        .and_then(|index| state.scene_models.get(index))
        .map(|scene| scene.name.clone());
    refresh_floor_projection(state);
    if state.choreography_settings_state.redraw_requested {
        state.draw_floor_request_count += 1;
    }
}

fn sync_dancers_projection(state: &mut ChoreoMainState) {
    let choreography = &state.choreography_settings_state.choreography;
    let selected_scene = state
        .choreography_settings_state
        .selected_scene
        .as_ref()
        .and_then(|selected| {
            choreography
                .scenes
                .iter()
                .find(|scene| scene.scene_id == selected.scene_id)
        });
    let selected_positions = selected_scene
        .map(|scene| {
            map_selected_positions_from_scene(scene, &state.floor_state.selected_positions)
        })
        .unwrap_or_default();

    state.dancers_state.global = dancers_state::DancersGlobalState {
        roles: choreography.roles.iter().map(map_role_state).collect(),
        dancers: choreography.dancers.iter().map(map_dancer_state).collect(),
        scenes: choreography
            .scenes
            .iter()
            .map(map_dancers_scene_state)
            .collect(),
        scene_views: choreography
            .scenes
            .iter()
            .map(map_dancers_scene_view_state)
            .collect(),
        selected_scene: selected_scene.map(map_dancers_scene_view_state),
        selected_positions: selected_positions.clone(),
        selected_positions_snapshot: selected_positions,
    };
}

fn sync_main_state_from_dancers(state: &mut ChoreoMainState) {
    let roles = state
        .dancers_state
        .roles
        .iter()
        .map(|role| {
            Rc::new(RoleModel {
                z_index: role.z_index,
                name: role.name.clone(),
                color: role.color.clone(),
            })
        })
        .collect::<Vec<_>>();
    let role_lookup = roles
        .iter()
        .map(|role| (role.name.to_ascii_lowercase(), role.clone()))
        .collect::<HashMap<_, _>>();
    let dancers = state
        .dancers_state
        .dancers
        .iter()
        .map(|dancer| {
            let role = role_lookup
                .get(&dancer.role.name.to_ascii_lowercase())
                .cloned()
                .or_else(|| roles.first().cloned())
                .unwrap_or_else(|| {
                    Rc::new(RoleModel {
                        z_index: dancer.role.z_index,
                        name: dancer.role.name.clone(),
                        color: dancer.role.color.clone(),
                    })
                });

            Rc::new(DancerModel {
                dancer_id: DancerId(dancer.dancer_id),
                role,
                name: dancer.name.clone(),
                shortcut: dancer.shortcut.clone(),
                color: dancer.color.clone(),
                icon: dancer.icon.clone(),
            })
        })
        .collect::<Vec<_>>();
    let dancer_lookup = dancers
        .iter()
        .map(|dancer| (dancer.dancer_id.0, dancer.clone()))
        .collect::<HashMap<_, _>>();
    let choreography = &mut state.choreography_settings_state.choreography;

    choreography.roles = roles;
    choreography.dancers = dancers;

    for scene in &mut choreography.scenes {
        update_scene_model_dancers(scene, &dancer_lookup);
    }

    sync_main_state_from_choreography_settings(state);
    sync_dancers_projection(state);
}

pub(crate) fn refresh_floor_projection(state: &mut ChoreoMainState) {
    let choreography = &state.choreography_settings_state.choreography;
    state.floor_state.floor_front = state.choreography_settings_state.floor_front;
    state.floor_state.floor_back = state.choreography_settings_state.floor_back;
    state.floor_state.floor_left = state.choreography_settings_state.floor_left;
    state.floor_state.floor_right = state.choreography_settings_state.floor_right;
    state.floor_state.snap_to_grid = state.choreography_settings_state.snap_to_grid;
    state.floor_state.grid_resolution = state.choreography_settings_state.grid_resolution();
    state.floor_state.choreography_name = choreography.name.clone();
    state.floor_state.show_grid_lines = state.choreography_settings_state.grid_lines;
    state.floor_state.positions_at_side = state.choreography_settings_state.positions_at_side;
    state.floor_state.show_legend = state.choreography_settings_state.show_legend;
    state.floor_state.draw_path_from = state.choreography_settings_state.draw_path_from;
    state.floor_state.draw_path_to = state.choreography_settings_state.draw_path_to;
    state.floor_state.floor_color = color_to_rgba(&state.choreography_settings_state.floor_color);
    state.floor_state.transparency = state.choreography_settings_state.transparency;
    state.floor_state.dancer_size = choreography.settings.dancer_size.max(1.0);
    state.floor_state.svg_path = state.svg_file_path.clone();

    let (previous_scene, current_scene, next_scene) = adjacent_scenes_for_audio_or_selected(
        &state.scene_models,
        state.selected_scene_index,
        state.audio_position_seconds,
    );

    state.floor_state.scene_name = current_scene
        .map(|scene| scene.name.clone())
        .unwrap_or_default();
    state.floor_state.source_positions = current_scene
        .map(|scene| map_scene_render_positions(scene, state.floor_state.transparency))
        .unwrap_or_default();
    state.floor_state.previous_source_positions = previous_scene
        .map(|scene| map_scene_render_positions(scene, state.floor_state.transparency))
        .unwrap_or_default();
    state.floor_state.next_source_positions = next_scene
        .map(|scene| map_scene_render_positions(scene, state.floor_state.transparency))
        .unwrap_or_default();
    state.floor_state.positions = state
        .floor_state
        .source_positions
        .iter()
        .map(|position| FloorPosition {
            x: position.x,
            y: position.y,
        })
        .collect();
    state.floor_state.interpolated_positions =
        build_interpolated_positions(current_scene, next_scene, state.audio_position_seconds);

    crate::floor::reducer::refresh_render_geometry(&mut state.floor_state);
}

fn adjacent_scenes_for_audio_or_selected(
    scenes: &[SceneModel],
    selected_index: Option<usize>,
    audio_position_seconds: f64,
) -> (
    Option<&SceneModel>,
    Option<&SceneModel>,
    Option<&SceneModel>,
) {
    for (index, window) in scenes.windows(2).enumerate() {
        let Some(current_timestamp) = parse_scene_timestamp(window[0].timestamp.as_deref()) else {
            continue;
        };
        let Some(next_timestamp) = parse_scene_timestamp(window[1].timestamp.as_deref()) else {
            continue;
        };
        if next_timestamp <= current_timestamp {
            continue;
        }
        if audio_position_seconds >= current_timestamp && audio_position_seconds <= next_timestamp {
            return (
                index
                    .checked_sub(1)
                    .and_then(|previous_index| scenes.get(previous_index)),
                scenes.get(index),
                scenes.get(index + 1),
            );
        }
    }

    let Some(index) = selected_index.filter(|index| *index < scenes.len()) else {
        return (None, None, None);
    };
    (
        index
            .checked_sub(1)
            .and_then(|previous_index| scenes.get(previous_index)),
        scenes.get(index),
        scenes.get(index + 1),
    )
}

fn map_role_state(role: &Rc<RoleModel>) -> dancers_state::RoleState {
    dancers_state::RoleState {
        name: role.name.clone(),
        color: role.color.clone(),
        z_index: role.z_index,
    }
}

fn map_dancer_state(dancer: &Rc<DancerModel>) -> dancers_state::DancerState {
    dancers_state::DancerState {
        dancer_id: dancer.dancer_id.0,
        role: map_role_state(&dancer.role),
        name: dancer.name.clone(),
        shortcut: dancer.shortcut.clone(),
        color: dancer.color.clone(),
        icon: dancer.icon.clone(),
    }
}

fn map_dancers_scene_state(scene: &SceneModel) -> dancers_state::SceneState {
    dancers_state::SceneState {
        positions: scene.positions.iter().map(map_position_state).collect(),
        variations: scene
            .variations
            .iter()
            .map(|variation| variation.iter().map(map_dancers_scene_state).collect())
            .collect(),
        current_variation: scene
            .current_variation
            .iter()
            .map(map_dancers_scene_state)
            .collect(),
    }
}

fn map_dancers_scene_view_state(scene: &SceneModel) -> dancers_state::SceneViewState {
    dancers_state::SceneViewState {
        positions: scene.positions.iter().map(map_position_state).collect(),
    }
}

fn map_position_state(position: &PositionModel) -> dancers_state::PositionState {
    dancers_state::PositionState {
        dancer_id: position.dancer.as_ref().map(|dancer| dancer.dancer_id.0),
        dancer_name: position.dancer.as_ref().map(|dancer| dancer.name.clone()),
    }
}

fn map_selected_positions_from_scene(
    scene: &SceneModel,
    selected_indexes: &[usize],
) -> Vec<dancers_state::PositionState> {
    selected_indexes
        .iter()
        .filter_map(|index| scene.positions.get(*index))
        .map(map_position_state)
        .collect()
}

fn update_scene_model_dancers(scene: &mut SceneModel, dancers: &HashMap<i32, Rc<DancerModel>>) {
    scene.positions.retain_mut(|position| {
        let Some(dancer_id) = position.dancer.as_ref().map(|dancer| dancer.dancer_id.0) else {
            return true;
        };

        if let Some(updated_dancer) = dancers.get(&dancer_id) {
            position.dancer = Some(updated_dancer.clone());
            true
        } else {
            false
        }
    });

    for variation in &mut scene.variations {
        for variation_scene in variation {
            update_scene_model_dancers(variation_scene, dancers);
        }
    }

    for variation_scene in &mut scene.current_variation {
        update_scene_model_dancers(variation_scene, dancers);
    }
}

fn map_scene_render_positions(scene: &SceneModel, transparency: f64) -> Vec<SceneRenderPosition> {
    scene
        .positions
        .iter()
        .map(|position| {
            let (
                dancer_key,
                dancer_name,
                shortcut,
                fill_color,
                border_color,
                text_color,
                has_dancer,
            ) = if let Some(dancer) = position.dancer.as_ref() {
                let fill_color =
                    apply_transparency_rgba(color_to_rgba(&dancer.color), transparency);
                let border_color =
                    apply_transparency_rgba(color_to_rgba(&dancer.role.color), transparency);
                (
                    dancer_key(dancer),
                    dancer.name.clone(),
                    dancer.shortcut.clone(),
                    fill_color,
                    border_color,
                    pick_black_or_white(fill_color),
                    true,
                )
            } else {
                let fill_color = apply_transparency_rgba([224, 224, 224, 255], transparency);
                (
                    None,
                    String::new(),
                    String::new(),
                    fill_color,
                    apply_transparency_rgba([120, 120, 120, 255], transparency),
                    [0, 0, 0, 255],
                    false,
                )
            };

            SceneRenderPosition {
                dancer_key,
                dancer_name,
                shortcut,
                x: position.x,
                y: position.y,
                curve1_x: position.curve1_x,
                curve1_y: position.curve1_y,
                curve2_x: position.curve2_x,
                curve2_y: position.curve2_y,
                fill_color,
                border_color,
                text_color,
                has_dancer,
            }
        })
        .collect()
}

fn build_interpolated_positions(
    current_scene: Option<&SceneModel>,
    next_scene: Option<&SceneModel>,
    audio_position_seconds: f64,
) -> Vec<FloorPosition> {
    let Some(current_scene) = current_scene else {
        return Vec::new();
    };
    let Some(next_scene) = next_scene else {
        return Vec::new();
    };
    let Some(current_timestamp) = parse_scene_timestamp(current_scene.timestamp.as_deref()) else {
        return Vec::new();
    };
    let Some(next_timestamp) = parse_scene_timestamp(next_scene.timestamp.as_deref()) else {
        return Vec::new();
    };
    let duration = next_timestamp - current_timestamp;
    if duration <= 0.0 {
        return Vec::new();
    }
    let progress = (audio_position_seconds - current_timestamp) / duration;
    if !(0.0..=1.0).contains(&progress) {
        return Vec::new();
    }

    let next_positions = build_positions_by_dancer_key(next_scene);
    current_scene
        .positions
        .iter()
        .map(|position| {
            let Some(dancer) = position.dancer.as_ref() else {
                return FloorPosition {
                    x: position.x,
                    y: position.y,
                };
            };
            let Some(next_position) = get_position_by_dancer_key(&next_positions, dancer) else {
                return FloorPosition {
                    x: position.x,
                    y: position.y,
                };
            };
            let (x, y) = interpolate_position(position, next_position, progress);
            FloorPosition { x, y }
        })
        .collect()
}

fn build_positions_by_dancer_key(scene: &SceneModel) -> HashMap<String, PositionModel> {
    let mut lookup = HashMap::new();
    for position in &scene.positions {
        let Some(dancer) = position.dancer.as_ref() else {
            continue;
        };
        if let Some(key) = dancer_key(dancer) {
            lookup.insert(key, position.clone());
        }
    }
    lookup
}

fn get_position_by_dancer_key<'a>(
    lookup: &'a HashMap<String, PositionModel>,
    dancer: &DancerModel,
) -> Option<&'a PositionModel> {
    let key = dancer_key(dancer)?;
    lookup.get(&key)
}

fn dancer_key(dancer: &DancerModel) -> Option<String> {
    if dancer.dancer_id.0 > 0 {
        return Some(format!("id:{}", dancer.dancer_id.0));
    }
    if !dancer.shortcut.trim().is_empty() {
        return Some(format!("shortcut:{}", dancer.shortcut));
    }
    if !dancer.name.trim().is_empty() {
        return Some(format!("name:{}", dancer.name));
    }
    None
}

fn interpolate_position(
    from_position: &PositionModel,
    to_position: &PositionModel,
    progress: f64,
) -> (f64, f64) {
    let Some(curve1_x) = from_position.curve1_x else {
        return (
            lerp(from_position.x, to_position.x, progress),
            lerp(from_position.y, to_position.y, progress),
        );
    };
    let Some(curve1_y) = from_position.curve1_y else {
        return (
            lerp(from_position.x, to_position.x, progress),
            lerp(from_position.y, to_position.y, progress),
        );
    };

    if let (Some(curve2_x), Some(curve2_y)) = (from_position.curve2_x, from_position.curve2_y) {
        return (
            cubic_bezier(from_position.x, curve1_x, curve2_x, to_position.x, progress),
            cubic_bezier(from_position.y, curve1_y, curve2_y, to_position.y, progress),
        );
    }

    (
        quadratic_bezier(from_position.x, curve1_x, to_position.x, progress),
        quadratic_bezier(from_position.y, curve1_y, to_position.y, progress),
    )
}

fn lerp(start: f64, end: f64, progress: f64) -> f64 {
    start + (end - start) * progress
}

fn quadratic_bezier(p0: f64, p1: f64, p2: f64, progress: f64) -> f64 {
    let inverse = 1.0 - progress;
    inverse * inverse * p0 + 2.0 * inverse * progress * p1 + progress * progress * p2
}

fn cubic_bezier(p0: f64, p1: f64, p2: f64, p3: f64, progress: f64) -> f64 {
    let inverse = 1.0 - progress;
    let inverse_squared = inverse * inverse;
    let progress_squared = progress * progress;
    inverse_squared * inverse * p0
        + 3.0 * inverse_squared * progress * p1
        + 3.0 * inverse * progress_squared * p2
        + progress_squared * progress * p3
}

fn color_to_rgba(color: &Color) -> [u8; 4] {
    [color.r, color.g, color.b, color.a]
}

fn apply_transparency_rgba(color: [u8; 4], transparency: f64) -> [u8; 4] {
    let opacity = (1.0 - transparency.clamp(0.0, 1.0)).clamp(0.0, 1.0);
    [
        color[0],
        color[1],
        color[2],
        (f64::from(color[3]) * opacity).round() as u8,
    ]
}

fn pick_black_or_white(color: [u8; 4]) -> [u8; 4] {
    let luminance = relative_luminance(color[0], color[1], color[2]);
    let contrast_black = (luminance + 0.05) / 0.05;
    let contrast_white = 1.05 / (luminance + 0.05);
    if contrast_white > contrast_black {
        [255, 255, 255, 255]
    } else {
        [0, 0, 0, 255]
    }
}

fn relative_luminance(red: u8, green: u8, blue: u8) -> f64 {
    let red = linearize_channel(f64::from(red) / 255.0);
    let green = linearize_channel(f64::from(green) / 255.0);
    let blue = linearize_channel(f64::from(blue) / 255.0);
    0.2126 * red + 0.7152 * green + 0.0722 * blue
}

fn linearize_channel(channel: f64) -> f64 {
    if channel <= 0.04045 {
        channel / 12.92
    } else {
        ((channel + 0.055) / 1.055).powf(2.4)
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

fn build_new_scene_name(scenes: &[super::state::SceneState]) -> String {
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
