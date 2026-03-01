use super::actions::ChoreographySettingsAction;
use super::create_state;
use super::reducer::reduce;

#[test]
fn update_positions_at_side_initializes_and_updates_global_value() {
    let mut state = create_state();

    reduce(&mut state, ChoreographySettingsAction::InitializePositionsAtSide(false));
    assert!(!state.positions_at_side);

    reduce(&mut state, ChoreographySettingsAction::UpdatePositionsAtSide(true));

    assert!(state.positions_at_side);
    assert!(state.preferences.positions_at_side);
    assert!(state.choreography.settings.positions_at_side);
    assert!(state.redraw_requested);
}
