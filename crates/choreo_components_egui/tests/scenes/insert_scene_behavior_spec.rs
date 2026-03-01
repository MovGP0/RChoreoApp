use super::actions::ScenesAction;
use super::choreography_with_scenes;
use super::create_state;
use super::reducer::reduce;
use super::scene_model;

#[test]
fn insert_scene_after_selected_and_select_new_scene() {
    let mut state = create_state();
    let choreography = choreography_with_scenes(
        "Test",
        vec![scene_model(1, "First", None, vec![]), scene_model(2, "Second", None, vec![])],
    );

    reduce(
        &mut state,
        ScenesAction::LoadScenes {
            choreography: Box::new(choreography),
        },
    );
    reduce(&mut state, ScenesAction::InsertScene { insert_after: true });

    assert_eq!(state.scenes.len(), 3);
    assert_eq!(state.scenes[1].name, "New Scene");
    assert_eq!(state.selected_scene.as_ref().map(|scene| scene.scene_id.0), Some(state.scenes[1].scene_id.0));
    assert_eq!(state.choreography.scenes.len(), 3);
}

#[test]
fn insert_scene_appends_when_none_selected() {
    let mut state = create_state();
    let choreography = choreography_with_scenes("Test", vec![scene_model(1, "First", None, vec![])]);

    reduce(
        &mut state,
        ScenesAction::LoadScenes {
            choreography: Box::new(choreography),
        },
    );
    state.selected_scene = None;

    reduce(&mut state, ScenesAction::InsertScene { insert_after: false });

    assert_eq!(state.scenes.len(), 2);
    assert_eq!(state.scenes[1].name, "New Scene");
}
