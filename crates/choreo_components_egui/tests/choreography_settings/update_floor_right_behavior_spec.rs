use super::actions::ChoreographySettingsAction;
use super::create_state;
use super::reducer::reduce;

#[test]
fn update_floor_right_clamps_to_minimum() {
    let mut state = create_state();

    reduce(&mut state, ChoreographySettingsAction::UpdateFloorRight(-10));

    assert_eq!(state.floor_right, 1);
    assert_eq!(state.choreography.floor.size_right, 1);
    assert!(state.redraw_requested);
}
