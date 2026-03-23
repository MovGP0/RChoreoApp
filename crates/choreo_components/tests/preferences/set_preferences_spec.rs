use super::actions::PreferencesAction;
use super::create_state;
use super::reducer::reduce;
use super::state::PreferenceWriteIntent;

#[test]
fn set_actions_store_values_and_queue_write_intents() {
    let mut state = create_state();

    reduce(
        &mut state,
        PreferencesAction::SetString {
            key: "last_opened".to_string(),
            value: "b.choreo".to_string(),
        },
    );
    reduce(
        &mut state,
        PreferencesAction::SetBool {
            key: "snap_to_grid".to_string(),
            value: true,
        },
    );

    assert_eq!(state.get_string("last_opened", ""), "b.choreo");
    assert!(state.get_bool("snap_to_grid", false));
    assert_eq!(state.pending_writes.len(), 2);
    assert_eq!(
        state.pending_writes[0],
        PreferenceWriteIntent::SetString {
            key: "last_opened".to_string(),
            value: "b.choreo".to_string(),
        }
    );
    assert_eq!(
        state.pending_writes[1],
        PreferenceWriteIntent::SetBool {
            key: "snap_to_grid".to_string(),
            value: true,
        }
    );
}
