use super::actions::PreferencesAction;
use super::create_state;
use super::reducer::reduce;
use super::state::PreferenceWriteIntent;

#[test]
fn set_actions_store_values_and_queue_write_intents() {
    macro_rules! check_eq {
        ($errors:expr, $left:expr, $right:expr) => {
            if $left != $right {
                $errors.push(format!(
                    "{} != {} (left = {:?}, right = {:?})",
                    stringify!($left),
                    stringify!($right),
                    $left,
                    $right
                ));
            }
        };
    }

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

    let mut errors = Vec::new();

    check_eq!(errors, state.get_string("last_opened", ""), "b.choreo");
    check_eq!(errors, state.get_bool("snap_to_grid", false), true);
    check_eq!(errors, state.pending_writes.len(), 2);
    check_eq!(
        errors,
        state.pending_writes[0],
        PreferenceWriteIntent::SetString {
            key: "last_opened".to_string(),
            value: "b.choreo".to_string(),
        }
    );
    check_eq!(
        errors,
        state.pending_writes[1],
        PreferenceWriteIntent::SetBool {
            key: "snap_to_grid".to_string(),
            value: true,
        }
    );

    assert!(
        errors.is_empty(),
        "Assertion failures:\n{}",
        errors.join("\n")
    );
}
