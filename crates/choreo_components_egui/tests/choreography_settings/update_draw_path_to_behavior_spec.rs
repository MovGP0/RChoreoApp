use super::actions::ChoreographySettingsAction;
use super::create_state;
use super::reducer::reduce;

#[test]
fn update_draw_path_to_initializes_and_updates_flag() {
    let mut state = create_state();

    reduce(&mut state, ChoreographySettingsAction::InitializeDrawPathTo(true));
    assert!(state.draw_path_to);

    reduce(&mut state, ChoreographySettingsAction::UpdateDrawPathTo(false));

    assert!(!state.draw_path_to);
    assert!(!state.preferences.draw_path_to);
    assert!(state.redraw_requested);
}
