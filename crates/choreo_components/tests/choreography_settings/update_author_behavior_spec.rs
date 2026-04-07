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
fn update_author_trims_and_sets_optional_author() {
    let mut state = create_state();

    reduce(
        &mut state,
        ChoreographySettingsAction::UpdateAuthor("  Jane Doe  ".to_string()),
    );

    let mut errors = Vec::new();

    check_eq!(errors, state.choreography.author.as_deref(), Some("Jane Doe"));
    check_eq!(errors, state.author, "Jane Doe");
    check!(errors, state.redraw_requested);

    assert_no_errors(errors);
}
