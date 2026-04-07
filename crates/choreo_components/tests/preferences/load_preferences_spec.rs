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

    let mut errors = Vec::new();

    check_eq!(errors, state.get_string("last_opened", ""), "a.choreo");
    check!(errors, state.get_bool("show_timestamps", false));
    check!(errors, state.pending_writes.is_empty());

    assert_no_errors(errors);
}
