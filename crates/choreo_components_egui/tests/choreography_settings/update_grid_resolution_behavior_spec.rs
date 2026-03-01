use super::actions::ChoreographySettingsAction;
use super::create_state;
use super::reducer::reduce;

#[test]
fn update_grid_resolution_clamps_to_supported_range() {
    let mut state = create_state();

    reduce(&mut state, ChoreographySettingsAction::UpdateGridResolution(42));

    assert_eq!(state.grid_resolution(), 16);
    assert_eq!(state.choreography.settings.resolution, 16);
    assert!(state.redraw_requested);
}
