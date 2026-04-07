use super::actions::PreferencesAction;
use super::create_state;
use super::reducer::reduce;
use super::state::PreferenceWriteIntent;

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

macro_rules! check {
    ($errors:expr, $condition:expr) => {
        if !$condition {
            $errors.push(format!("condition failed: {}", stringify!($condition)));
        }
    };
}

fn assert_no_errors(errors: Vec<String>) {
    assert!(
        errors.is_empty(),
        "Assertion failures:\n{}",
        errors.join("\n")
    );
}

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

    let mut errors = Vec::new();

    check_eq!(errors, state.get_string("music", ""), "song.mp3");
    check!(errors, !state.get_bool("snap", false));
    check_eq!(
        errors,
        state.pending_writes.last().cloned(),
        Some(PreferenceWriteIntent::Remove {
            key: "snap".to_string(),
        })
    );

    assert_no_errors(errors);
}
