use super::actions::ChoreographySettingsAction;
use super::create_state;
use super::reducer::reduce;

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
fn update_grid_lines_sets_value_and_redraw() {
    let mut state = create_state();

    reduce(
        &mut state,
        ChoreographySettingsAction::UpdateGridLines(true),
    );

    let mut errors = Vec::new();

    check!(errors, state.grid_lines);
    check!(errors, state.choreography.settings.grid_lines);
    check!(errors, state.redraw_requested);

    assert_no_errors(errors);
}
