use crate::audio_player::actions::AudioPlayerAction;
use crate::choreo_main::actions::ChoreoMainAction;
use crate::choreo_main::actions::OpenAudioRequested;
use crate::choreo_main::actions::OpenChoreoRequested;
use crate::choreo_main::state::ChoreoMainState;
use crate::choreography_settings::actions::ChoreographySettingsAction;
use crate::floor::actions::FloorAction;
use crate::material::components::drawer_host::actions::DrawerHostAction;
use crate::scenes::actions::ScenesAction;

#[must_use]
pub fn map_drawer_host_action(
    action: DrawerHostAction,
    state: &ChoreoMainState,
) -> Vec<ChoreoMainAction> {
    match action {
        DrawerHostAction::OverlayClicked {
            close_left,
            close_right,
            close_top: _,
            close_bottom: _,
        } => {
            let mut actions = Vec::new();
            if close_left && state.is_nav_open {
                actions.push(ChoreoMainAction::CloseNav);
            }
            if close_right && state.is_choreography_settings_open {
                actions.push(ChoreoMainAction::CloseSettings);
            }
            actions
        }
    }
}

#[must_use]
pub fn map_floor_host_action(action: FloorAction) -> ChoreoMainAction {
    ChoreoMainAction::FloorAction(action)
}

#[must_use]
pub fn map_audio_host_action(action: AudioPlayerAction) -> Vec<ChoreoMainAction> {
    let mut mapped = vec![ChoreoMainAction::AudioPlayerAction(action.clone())];
    match action {
        AudioPlayerAction::SeekToPosition { position }
        | AudioPlayerAction::PositionDragCompleted { position } => {
            mapped.push(ChoreoMainAction::UpdateAudioPosition { seconds: position });
        }
        AudioPlayerAction::LinkSceneToPosition => {
            mapped.push(ChoreoMainAction::LinkSelectedSceneToAudioPosition);
        }
        _ => {}
    }
    mapped
}

#[must_use]
pub fn map_choreography_settings_action(action: ChoreographySettingsAction) -> ChoreoMainAction {
    ChoreoMainAction::ChoreographySettingsAction(action)
}

#[must_use]
pub fn map_scene_pane_action(action: ScenesAction) -> Option<ChoreoMainAction> {
    match action {
        ScenesAction::RequestOpenChoreography => {
            Some(ChoreoMainAction::RequestOpenChoreo(OpenChoreoRequested {
                file_path: None,
                file_name: None,
                contents: String::new(),
            }))
        }
        ScenesAction::RequestSaveChoreography => Some(ChoreoMainAction::RequestSaveChoreo),
        ScenesAction::NavigateToSettings => Some(ChoreoMainAction::NavigateToSettings),
        ScenesAction::NavigateToDancerSettings => Some(ChoreoMainAction::NavigateToDancers),
        ScenesAction::UpdateSearchText(value) => {
            Some(ChoreoMainAction::UpdateSceneSearchText(value))
        }
        ScenesAction::InsertScene { insert_after } => {
            Some(ChoreoMainAction::InsertScene { insert_after })
        }
        ScenesAction::OpenDeleteSceneDialog => Some(ChoreoMainAction::DeleteSelectedScene),
        ScenesAction::SelectScene { index } => Some(ChoreoMainAction::SelectScene { index }),
        ScenesAction::LoadScenes { .. }
        | ScenesAction::ReloadScenes
        | ScenesAction::SelectSceneFromAudioPosition { .. }
        | ScenesAction::ApplyPlacementModeForSelected
        | ScenesAction::SyncShowTimestampsFromChoreography
        | ScenesAction::UpdateShowTimestamps(_)
        | ScenesAction::CancelDeleteSceneDialog
        | ScenesAction::ConfirmDeleteSceneDialog
        | ScenesAction::OpenCopyScenePositionsDialog
        | ScenesAction::CancelCopyScenePositionsDialog
        | ScenesAction::ConfirmCopyScenePositionsDialog { .. }
        | ScenesAction::OpenChoreography { .. }
        | ScenesAction::SaveChoreography
        | ScenesAction::ClearEphemeralOutputs => None,
    }
}

#[must_use]
pub fn top_bar_nav_action(is_nav_open: bool) -> ChoreoMainAction {
    if is_nav_open {
        ChoreoMainAction::CloseNav
    } else {
        ChoreoMainAction::ToggleNav
    }
}

#[must_use]
pub fn top_bar_settings_action(is_settings_open: bool) -> ChoreoMainAction {
    if is_settings_open {
        ChoreoMainAction::CloseSettings
    } else {
        ChoreoMainAction::OpenSettings
    }
}

#[must_use]
pub fn top_bar_open_audio_action() -> ChoreoMainAction {
    ChoreoMainAction::RequestOpenAudio(OpenAudioRequested {
        file_path: String::new(),
        trace_context: None,
    })
}
