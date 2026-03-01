use super::actions::PreferencesAction;
use super::create_state;
use super::reducer::reduce;
use super::state::PreferenceWriteIntent;

#[test]
fn remove_deletes_string_and_bool_entries() {
    let mut state = create_state();

    reduce(
        &mut state,
        PreferencesAction::SetString {
            key: "music".to_string(),
            value: "song.mp3".to_string(),
        },
    );
    reduce(
        &mut state,
        PreferencesAction::SetBool {
            key: "snap".to_string(),
            value: true,
        },
    );

    reduce(
        &mut state,
        PreferencesAction::Remove {
            key: "snap".to_string(),
        },
    );

    assert_eq!(state.get_string("music", ""), "song.mp3");
    assert!(!state.get_bool("snap", false));
    assert_eq!(
        state.pending_writes.last().cloned(),
        Some(PreferenceWriteIntent::Remove {
            key: "snap".to_string(),
        })
    );
}
