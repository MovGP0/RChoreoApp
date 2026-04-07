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
fn update_snap_to_grid_initializes_and_updates_state() {
    let mut state = create_state();
    let mut errors = Vec::new();

    reduce(
        &mut state,
        ChoreographySettingsAction::InitializeSnapToGrid(false),
    );
    check!(errors, !state.snap_to_grid);

    reduce(
        &mut state,
        ChoreographySettingsAction::UpdateSnapToGrid(true),
    );

    check!(errors, state.snap_to_grid);
    check!(errors, state.preferences.snap_to_grid);
    check!(errors, state.choreography.settings.snap_to_grid);
    check!(errors, state.redraw_requested);

    assert_no_errors(errors);
}
