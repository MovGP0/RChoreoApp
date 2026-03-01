use super::actions::ChoreographySettingsAction;
use super::create_state;
use super::reducer::reduce;

#[test]
fn update_show_timestamps_initializes_updates_state_and_emits_event() {
    let mut state = create_state();

    reduce(&mut state, ChoreographySettingsAction::InitializeShowTimestamps(false));
    assert!(!state.show_timestamps);

    reduce(&mut state, ChoreographySettingsAction::UpdateShowTimestamps(true));

    assert!(state.show_timestamps);
    assert!(state.preferences.show_timestamps);
    assert!(state.choreography.settings.show_timestamps);
    assert!(state.redraw_requested);
    assert_eq!(state.last_show_timestamps_event.map(|event| event.is_enabled), Some(true));

    state.set_scene_timestamp_parts(1, 2, 39);
    assert!((state.scene_timestamp_seconds - 62.03).abs() < 0.0001);

    reduce(&mut state, ChoreographySettingsAction::ClearEphemeralOutputs);
    assert!(!state.redraw_requested);
    assert!(state.last_show_timestamps_event.is_none());
}
