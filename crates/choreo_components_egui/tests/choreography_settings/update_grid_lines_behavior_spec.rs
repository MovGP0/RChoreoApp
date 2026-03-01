use super::actions::ChoreographySettingsAction;
use super::create_state;
use super::reducer::reduce;

#[test]
fn update_grid_lines_sets_value_and_redraw() {
    let mut state = create_state();

    reduce(&mut state, ChoreographySettingsAction::UpdateGridLines(true));

    assert!(state.grid_lines);
    assert!(state.choreography.settings.grid_lines);
    assert!(state.redraw_requested);
}
