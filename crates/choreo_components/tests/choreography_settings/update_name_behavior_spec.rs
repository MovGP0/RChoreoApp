use super::actions::ChoreographySettingsAction;
use super::create_state;
use super::reducer::reduce;

#[test]
fn update_name_trims_and_sets_name() {
    let mut state = create_state();

    reduce(
        &mut state,
        ChoreographySettingsAction::UpdateName("  Choreo Name  ".to_string()),
    );

    assert_eq!(state.choreography.name, "Choreo Name");
    assert_eq!(state.name, "Choreo Name");
    assert!(state.redraw_requested);
}
