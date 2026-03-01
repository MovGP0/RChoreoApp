use super::actions::ChoreographySettingsAction;
use super::create_state;
use super::reducer::reduce;

#[test]
fn update_draw_path_from_initializes_and_updates_flag() {
    let mut state = create_state();

    reduce(&mut state, ChoreographySettingsAction::InitializeDrawPathFrom(true));
    assert!(state.draw_path_from);

    reduce(&mut state, ChoreographySettingsAction::UpdateDrawPathFrom(false));

    assert!(!state.draw_path_from);
    assert!(!state.preferences.draw_path_from);
    assert!(state.redraw_requested);
}
