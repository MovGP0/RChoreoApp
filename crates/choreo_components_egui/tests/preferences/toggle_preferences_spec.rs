use super::actions::PreferencesAction;
use super::create_state;
use super::reducer::reduce;

#[test]
fn toggle_bool_flips_from_default_and_existing_values() {
    let mut state = create_state();

    reduce(
        &mut state,
        PreferencesAction::ToggleBool {
            key: "show_timestamps".to_string(),
            default: false,
        },
    );
    assert!(state.get_bool("show_timestamps", false));

    reduce(
        &mut state,
        PreferencesAction::ToggleBool {
            key: "show_timestamps".to_string(),
            default: false,
        },
    );
    assert!(!state.get_bool("show_timestamps", false));
}
