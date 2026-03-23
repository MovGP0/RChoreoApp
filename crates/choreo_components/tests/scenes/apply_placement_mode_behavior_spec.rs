use super::actions::ScenesAction;
use super::build_dancer;
use super::build_position;
use super::choreography_with_scenes;
use super::create_state;
use super::reducer::reduce;
use super::scene_model;

#[test]
fn apply_placement_mode_enables_when_positions_are_less_than_dancers() {
    let mut state = create_state();
    let scene = scene_model(1, "First", None, vec![build_position(0.0, 0.0)]);
    let mut choreography = choreography_with_scenes("Test", vec![scene]);
    choreography.dancers = vec![build_dancer(1, "A"), build_dancer(2, "B")];

    reduce(
        &mut state,
        ScenesAction::LoadScenes {
            choreography: Box::new(choreography),
        },
    );
    reduce(&mut state, ScenesAction::ApplyPlacementModeForSelected);

    assert!(state.is_place_mode);
}

#[test]
fn apply_placement_mode_disables_when_selection_is_cleared() {
    let mut state = create_state();
    let scene = scene_model(1, "First", None, vec![]);
    let mut choreography = choreography_with_scenes("Test", vec![scene]);
    choreography.dancers = vec![build_dancer(1, "A")];

    reduce(
        &mut state,
        ScenesAction::LoadScenes {
            choreography: Box::new(choreography),
        },
    );
    state.selected_scene = None;

    reduce(&mut state, ScenesAction::ApplyPlacementModeForSelected);

    assert!(!state.is_place_mode);
}
