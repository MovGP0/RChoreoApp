use super::actions::ChoreographySettingsAction;
use super::create_state;
use super::reducer::reduce;

#[test]
fn update_subtitle_trims_and_sets_optional_subtitle() {
    let mut state = create_state();

    reduce(
        &mut state,
        ChoreographySettingsAction::UpdateSubtitle("  Subtitle  ".to_string()),
    );

    assert_eq!(state.choreography.subtitle.as_deref(), Some("Subtitle"));
    assert_eq!(state.subtitle, "Subtitle");
    assert!(state.redraw_requested);
}
