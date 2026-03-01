use super::actions::ChoreographySettingsAction;
use super::create_state;
use super::reducer::reduce;

#[test]
fn update_comment_trims_and_sets_optional_comment() {
    let mut state = create_state();

    reduce(
        &mut state,
        ChoreographySettingsAction::UpdateComment("  comment text  ".to_string()),
    );

    assert_eq!(state.choreography.comment.as_deref(), Some("comment text"));
    assert_eq!(state.comment, "comment text");
    assert!(state.redraw_requested);
}
