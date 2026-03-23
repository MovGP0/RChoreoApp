use super::actions::ChoreographySettingsAction;
use super::create_state;
use super::reducer::reduce;

#[test]
fn update_author_trims_and_sets_optional_author() {
    let mut state = create_state();

    reduce(
        &mut state,
        ChoreographySettingsAction::UpdateAuthor("  Jane Doe  ".to_string()),
    );

    assert_eq!(state.choreography.author.as_deref(), Some("Jane Doe"));
    assert_eq!(state.author, "Jane Doe");
    assert!(state.redraw_requested);
}
