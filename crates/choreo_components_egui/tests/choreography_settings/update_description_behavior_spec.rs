use super::actions::ChoreographySettingsAction;
use super::create_state;
use super::reducer::reduce;

#[test]
fn update_description_trims_and_sets_optional_description() {
    let mut state = create_state();

    reduce(
        &mut state,
        ChoreographySettingsAction::UpdateDescription("  description text  ".to_string()),
    );

    assert_eq!(state.choreography.description.as_deref(), Some("description text"));
    assert_eq!(state.description, "description text");
    assert!(state.redraw_requested);
}
