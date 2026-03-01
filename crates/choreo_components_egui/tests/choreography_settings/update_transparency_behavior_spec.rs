use super::actions::ChoreographySettingsAction;
use super::create_state;
use super::reducer::reduce;

#[test]
fn update_transparency_clamps_to_zero_and_one() {
    let mut state = create_state();

    reduce(&mut state, ChoreographySettingsAction::UpdateTransparency(2.0));

    assert!((state.transparency - 1.0).abs() < 0.0001);
    assert!((state.choreography.settings.transparency - 1.0).abs() < 0.0001);
    assert!(state.redraw_requested);
}
