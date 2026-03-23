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
    assert_eq!(
        state
            .delete_scene_dialog_scene
            .as_ref()
            .map(|scene| (scene.scene_id, scene.name.as_str())),
        Some((choreo_master_mobile_json::SceneId(1), "Alpha"))
    );
}

#[test]
fn cancel_delete_scene_dialog_closes_dialog() {
    let mut state = create_state();
    state.show_delete_scene_dialog = true;
    state.delete_scene_dialog_scene = Some(super::state::SceneItemState::new(
        choreo_master_mobile_json::SceneId(1),
        "Alpha",
        choreo_master_mobile_json::Color::transparent(),
    ));

    reduce(&mut state, ScenesAction::CancelDeleteSceneDialog);

    assert!(!state.show_delete_scene_dialog);
    assert_eq!(state.delete_scene_dialog_scene, None);
}

#[test]
fn confirm_delete_scene_dialog_requests_delete_and_closes_dialog() {
    let mut state = create_state();
    state.show_delete_scene_dialog = true;
    state.delete_scene_requested = false;
    state.delete_scene_dialog_scene = Some(super::state::SceneItemState::new(
        choreo_master_mobile_json::SceneId(7),
        "Alpha",
        choreo_master_mobile_json::Color::transparent(),
    ));

    reduce(&mut state, ScenesAction::ConfirmDeleteSceneDialog);

    assert!(!state.show_delete_scene_dialog);
    assert_eq!(state.delete_scene_dialog_scene, None);
    assert!(state.delete_scene_requested);
    assert_eq!(
        state.delete_scene_requested_scene_id,
        Some(choreo_master_mobile_json::SceneId(7))
    );
}

#[test]
fn confirm_delete_scene_dialog_keeps_original_target_when_selection_changes() {
    let mut state = create_state();
    let choreography = choreography_with_scenes(
        "Test",
        vec![
            scene_model(1, "Alpha", None, vec![]),
            scene_model(2, "Beta", None, vec![]),
        ],
    );
    reduce(
        &mut state,
        ScenesAction::LoadScenes {
            choreography: Box::new(choreography),
        },
    );
    reduce(&mut state, ScenesAction::OpenDeleteSceneDialog);
    reduce(&mut state, ScenesAction::SelectScene { index: 1 });

    reduce(&mut state, ScenesAction::ConfirmDeleteSceneDialog);

    assert_eq!(
        state.delete_scene_requested_scene_id,
        Some(choreo_master_mobile_json::SceneId(1))
    );
}
