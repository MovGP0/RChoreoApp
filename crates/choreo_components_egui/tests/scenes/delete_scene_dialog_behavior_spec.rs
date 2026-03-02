use super::actions::ScenesAction;
use super::choreography_with_scenes;
use super::create_state;
use super::reducer::reduce;
use super::scene_model;

#[test]
fn show_delete_scene_dialog_opens_for_selected_scene() {
    let mut state = create_state();
    let choreography =
        choreography_with_scenes("Test", vec![scene_model(1, "Alpha", None, vec![])]);
    reduce(
        &mut state,
        ScenesAction::LoadScenes {
            choreography: Box::new(choreography),
        },
    );

    reduce(&mut state, ScenesAction::OpenDeleteSceneDialog);

    assert!(state.show_delete_scene_dialog);
}

#[test]
fn cancel_delete_scene_dialog_closes_dialog() {
    let mut state = create_state();
    state.show_delete_scene_dialog = true;

    reduce(&mut state, ScenesAction::CancelDeleteSceneDialog);

    assert!(!state.show_delete_scene_dialog);
}

#[test]
fn confirm_delete_scene_dialog_requests_delete_and_closes_dialog() {
    let mut state = create_state();
    state.show_delete_scene_dialog = true;
    state.delete_scene_requested = false;

    reduce(&mut state, ScenesAction::ConfirmDeleteSceneDialog);

    assert!(!state.show_delete_scene_dialog);
    assert!(state.delete_scene_requested);
}
