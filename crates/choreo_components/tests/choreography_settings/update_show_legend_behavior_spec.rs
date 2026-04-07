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
fn update_show_legend_initializes_and_updates_flag() {
    let mut state = create_state();
    let mut errors = Vec::new();

    reduce(
        &mut state,
        ChoreographySettingsAction::InitializeShowLegend(true),
    );
    check!(errors, state.show_legend);

    reduce(
        &mut state,
        ChoreographySettingsAction::UpdateShowLegend(false),
    );

    check_eq!(errors, state.show_legend, false);
    check_eq!(errors, state.preferences.show_legend, false);
    check!(errors, state.redraw_requested);

    assert_no_errors(errors);
}
