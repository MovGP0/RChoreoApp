use super::actions::PreferencesAction;
use super::create_state;
use super::reducer::reduce;

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
fn clear_pending_writes_only_clears_intents() {
    let mut state = create_state();

    reduce(
        &mut state,
        PreferencesAction::SetBool {
            key: "legend".to_string(),
            value: true,
        },
    );
    let mut errors = Vec::new();

    check_eq!(errors, state.pending_writes.len(), 1);

    reduce(&mut state, PreferencesAction::ClearPendingWrites);

    check_eq!(errors, state.pending_writes.len(), 0);
    check!(errors, state.get_bool("legend", false));

    assert_no_errors(errors);
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

    let mut errors = Vec::new();

    check_eq!(errors, state.get_string("last_opened", ""), "");
    check!(errors, !state.get_bool("legend", false));
    check!(errors, state.pending_writes.is_empty());

    assert_no_errors(errors);
}
