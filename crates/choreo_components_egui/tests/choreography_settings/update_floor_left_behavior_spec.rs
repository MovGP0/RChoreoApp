use super::actions::ChoreographySettingsAction;
use super::create_state;
use super::reducer::reduce;

#[test]
fn update_floor_left_clamps_to_maximum() {
    let mut state = create_state();

    reduce(&mut state, ChoreographySettingsAction::UpdateFloorLeft(101));

    assert_eq!(state.floor_left, 100);
    assert_eq!(state.choreography.floor.size_left, 100);
    assert!(state.redraw_requested);
}
