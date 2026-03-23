use super::actions::ScenesAction;
use super::choreography_with_scenes;
use super::create_state;
use super::reducer::reduce;
use super::scene_model;

#[test]
fn copy_scene_positions_dialog_emits_copy_or_keep_decisions() {
    let mut state = create_state();
    reduce(
        &mut state,
        ScenesAction::LoadScenes {
            choreography: Box::new(choreography_with_scenes(
                "Show",
                vec![scene_model(1, "Alpha", None, vec![])],
            )),
        },
    );

    reduce(&mut state, ScenesAction::OpenCopyScenePositionsDialog);
    assert!(state.show_copy_scene_positions_dialog);

    reduce(
        &mut state,
        ScenesAction::ConfirmCopyScenePositionsDialog {
            copy_positions: true,
        },
    );
    assert!(!state.show_copy_scene_positions_dialog);
    assert_eq!(state.copy_scene_positions_decision, Some(true));

    reduce(&mut state, ScenesAction::OpenCopyScenePositionsDialog);
    reduce(
        &mut state,
        ScenesAction::ConfirmCopyScenePositionsDialog {
            copy_positions: false,
        },
    );
    assert_eq!(state.copy_scene_positions_decision, Some(false));
}
