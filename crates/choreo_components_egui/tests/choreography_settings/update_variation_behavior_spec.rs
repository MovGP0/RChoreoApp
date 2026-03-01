use super::actions::ChoreographySettingsAction;
use super::create_state;
use super::reducer::reduce;

#[test]
fn update_variation_trims_and_sets_optional_variation() {
    let mut state = create_state();

    reduce(
        &mut state,
        ChoreographySettingsAction::UpdateVariation("  alt  ".to_string()),
    );

    assert_eq!(state.choreography.variation.as_deref(), Some("alt"));
    assert_eq!(state.variation, "alt");
    assert!(state.redraw_requested);
}
