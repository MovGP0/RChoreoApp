use super::actions::ChoreographySettingsAction;
use super::create_state;
use super::reducer::reduce;

#[test]
fn update_snap_to_grid_initializes_and_updates_state() {
    let mut state = create_state();

    reduce(&mut state, ChoreographySettingsAction::InitializeSnapToGrid(false));
    assert!(!state.snap_to_grid);

    reduce(&mut state, ChoreographySettingsAction::UpdateSnapToGrid(true));

    assert!(state.snap_to_grid);
    assert!(state.preferences.snap_to_grid);
    assert!(state.choreography.settings.snap_to_grid);
    assert!(state.redraw_requested);
}
