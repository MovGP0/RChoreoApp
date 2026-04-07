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
fn update_transparency_clamps_to_zero_and_one() {
    let mut state = create_state();

    reduce(
        &mut state,
        ChoreographySettingsAction::UpdateTransparency(2.0),
    );

    let mut errors = Vec::new();

    check_eq!(errors, state.transparency, 1.0);
    check_eq!(errors, state.choreography.settings.transparency, 1.0);
    check!(errors, state.redraw_requested);

    assert_no_errors(errors);
}
