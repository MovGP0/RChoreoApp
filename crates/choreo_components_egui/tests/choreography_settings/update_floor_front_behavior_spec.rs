use super::actions::ChoreographySettingsAction;
use super::create_state;
use super::reducer::reduce;

#[test]
fn update_floor_front_clamps_to_minimum() {
    let mut state = create_state();

    reduce(&mut state, ChoreographySettingsAction::UpdateFloorFront(0));

    assert_eq!(state.floor_front, 1);
    assert_eq!(state.choreography.floor.size_front, 1);
    assert!(state.redraw_requested);
}
