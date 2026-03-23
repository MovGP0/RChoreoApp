use super::actions::PreferencesAction;
use super::create_state;
use super::reducer::reduce;

#[test]
fn load_actions_populate_state_without_pending_writes() {
    let mut state = create_state();

    reduce(
        &mut state,
        PreferencesAction::LoadString {
            key: "last_opened".to_string(),
            value: "a.choreo".to_string(),
        },
    );
    reduce(
        &mut state,
        PreferencesAction::LoadBool {
            key: "show_timestamps".to_string(),
            value: true,
        },
    );

    assert_eq!(state.get_string("last_opened", ""), "a.choreo");
    assert!(state.get_bool("show_timestamps", false));
    assert!(state.pending_writes.is_empty());
}
