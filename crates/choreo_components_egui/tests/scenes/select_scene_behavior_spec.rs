use super::actions::ScenesAction;
use super::choreography_with_scenes;
use super::create_state;
use super::reducer::reduce;
use super::scene_model;

#[test]
fn select_scene_updates_selected_scene_and_emits_flags() {
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

    reduce(&mut state, ScenesAction::SelectScene { index: 1 });

    assert_eq!(state.selected_scene.as_ref().map(|scene| scene.name.as_str()), Some("Second"));
    assert!(state.selected_scene_changed);
    assert!(state.redraw_floor_requested);
}

#[test]
fn select_scene_ignores_out_of_range_index() {
    let mut state = create_state();
    let choreography = choreography_with_scenes("Test", vec![scene_model(1, "First", None, vec![])]);

    reduce(
        &mut state,
        ScenesAction::LoadScenes {
            choreography: Box::new(choreography),
        },
    );
    state.clear_ephemeral_outputs();

    reduce(&mut state, ScenesAction::SelectScene { index: 10 });

    assert_eq!(state.selected_scene.as_ref().map(|scene| scene.name.as_str()), Some("First"));
    assert!(!state.selected_scene_changed);
    assert!(!state.redraw_floor_requested);
}
