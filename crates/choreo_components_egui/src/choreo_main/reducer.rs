use super::actions::ChoreoMainAction;
use super::state::ChoreoMainState;
use super::state::InteractionMode;
use super::state::InteractionStateMachineState;
use super::state::MainContent;

pub fn reduce(state: &mut ChoreoMainState, action: ChoreoMainAction) {
    match action {
        ChoreoMainAction::Initialize => {
            state.content = MainContent::Main;
            state.is_nav_open = false;
            state.is_choreography_settings_open = false;
            state.is_audio_player_open = false;
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
        }
        ChoreoMainAction::SelectScene { index } => {
            select_scene_internal(state, index, false);
        }
        ChoreoMainAction::UpdateAudioPosition { seconds } => {
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
            }
        }
        ChoreoMainAction::DancersAction(action) => {
            crate::dancers::reducer::reduce(&mut state.dancers_state, action);
        }
        ChoreoMainAction::ClearOutgoingCommands => {
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
