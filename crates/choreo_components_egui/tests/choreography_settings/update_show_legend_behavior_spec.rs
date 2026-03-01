use super::actions::ChoreographySettingsAction;
use super::create_state;
use super::reducer::reduce;

#[test]
fn update_show_legend_initializes_and_updates_flag() {
    let mut state = create_state();

    reduce(&mut state, ChoreographySettingsAction::InitializeShowLegend(true));
    assert!(state.show_legend);

    reduce(&mut state, ChoreographySettingsAction::UpdateShowLegend(false));

    assert!(!state.show_legend);
    assert!(!state.preferences.show_legend);
    assert!(state.redraw_requested);
}
