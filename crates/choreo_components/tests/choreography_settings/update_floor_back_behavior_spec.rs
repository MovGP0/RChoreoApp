use super::actions::ChoreographySettingsAction;
use super::create_state;
use super::reducer::reduce;

#[test]
fn update_floor_back_clamps_to_maximum() {
    let mut state = create_state();

    reduce(&mut state, ChoreographySettingsAction::UpdateFloorBack(999));

    assert_eq!(state.floor_back, 100);
    assert_eq!(state.choreography.floor.size_back, 100);
    assert!(state.redraw_requested);
}
