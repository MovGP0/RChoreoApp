use super::actions::PreferencesAction;
use super::create_state;
use super::reducer::reduce;

#[test]
fn toggle_bool_flips_from_default_and_existing_values() {
    macro_rules! check {
        ($errors:expr, $condition:expr) => {
            if !$condition {
                $errors.push(format!(
                    "assertion failed: {}",
                    stringify!($condition)
                ));
            }
        };
    }

    let mut state = create_state();
    let mut errors = Vec::new();

    reduce(
        &mut state,
        PreferencesAction::ToggleBool {
            key: "show_timestamps".to_string(),
            default: false,
        },
    );
    check!(errors, state.get_bool("show_timestamps", false));

    reduce(
        &mut state,
        PreferencesAction::ToggleBool {
            key: "show_timestamps".to_string(),
            default: false,
        },
    );
    check!(errors, !state.get_bool("show_timestamps", false));

    assert!(
        errors.is_empty(),
        "Assertion failures:\n{}",
        errors.join("\n")
    );
}
