use super::actions::PreferencesAction;
use super::create_state;
use super::reducer::reduce;

#[test]
fn clear_pending_writes_only_clears_intents() {
    let mut state = create_state();

    reduce(
        &mut state,
        PreferencesAction::SetBool {
            key: "legend".to_string(),
            value: true,
        },
    );
    assert_eq!(state.pending_writes.len(), 1);

    reduce(&mut state, PreferencesAction::ClearPendingWrites);

    assert_eq!(state.pending_writes.len(), 0);
    assert!(state.get_bool("legend", false));
}

#[test]
fn clear_all_resets_values_and_pending_writes() {
    let mut state = create_state();

    reduce(
        &mut state,
        PreferencesAction::SetString {
            key: "last_opened".to_string(),
            value: "c.choreo".to_string(),
        },
    );
    reduce(
        &mut state,
        PreferencesAction::SetBool {
            key: "legend".to_string(),
            value: true,
        },
    );

    reduce(&mut state, PreferencesAction::ClearAll);

    assert_eq!(state.get_string("last_opened", ""), "");
    assert!(!state.get_bool("legend", false));
    assert!(state.pending_writes.is_empty());
}
