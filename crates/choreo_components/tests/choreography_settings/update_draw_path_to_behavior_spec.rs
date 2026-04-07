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

fn assert_no_errors(errors: Vec<String>) {
    assert!(
        errors.is_empty(),
        "Assertion failures:\n{}",
        errors.join("\n")
    );
}

#[test]
fn update_draw_path_to_initializes_and_updates_flag() {
    let mut state = create_state();
    let mut errors = Vec::new();

    reduce(
        &mut state,
        ChoreographySettingsAction::InitializeDrawPathTo(true),
    );
    check_eq!(errors, state.draw_path_to, true);

    reduce(
        &mut state,
        ChoreographySettingsAction::UpdateDrawPathTo(false),
    );

    check_eq!(errors, state.draw_path_to, false);
    check_eq!(errors, state.preferences.draw_path_to, false);
    check_eq!(errors, state.redraw_requested, true);

    assert_no_errors(errors);
}
