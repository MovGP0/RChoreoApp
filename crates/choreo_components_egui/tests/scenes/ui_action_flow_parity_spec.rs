use super::actions::ScenesAction;
use super::create_state;
use super::ui::scene_pane_action_flow;

#[test]
fn scene_pane_action_flow_matches_original_controls_and_caps() {
    let mut state = create_state();
    state.can_delete_scene = true;
    state.can_save_choreo = true;
    state.can_navigate_to_settings = true;
    state.can_navigate_to_dancer_settings = true;

    let actions = scene_pane_action_flow(&state);

    assert_eq!(
        actions,
        vec![
            ScenesAction::InsertScene {
                insert_after: false
            },
            ScenesAction::InsertScene { insert_after: true },
            ScenesAction::OpenDeleteSceneDialog,
            ScenesAction::RequestOpenChoreography,
            ScenesAction::RequestSaveChoreography,
            ScenesAction::NavigateToSettings,
            ScenesAction::NavigateToDancerSettings,
        ]
    );
}

#[test]
fn scene_pane_action_flow_excludes_non_parity_controls() {
    let state = create_state();
    let actions = scene_pane_action_flow(&state);

    assert!(!actions.contains(&ScenesAction::OpenCopyScenePositionsDialog));
    assert!(
        !actions
            .iter()
            .any(|action| { matches!(action, ScenesAction::UpdateShowTimestamps(_)) })
    );
}
