use super::actions::ChoreographySettingsAction;
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
fn update_comment_trims_and_sets_optional_comment() {
    let mut state = create_state();

    reduce(
        &mut state,
        ChoreographySettingsAction::UpdateComment("  comment text  ".to_string()),
    );

    let mut errors = Vec::new();

    check_eq!(errors, state.choreography.comment.as_deref(), Some("comment text"));
    check_eq!(errors, state.comment, "comment text");
    check!(errors, state.redraw_requested);

    assert_no_errors(errors);
}
