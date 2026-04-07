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
fn update_positions_at_side_initializes_and_updates_global_value() {
    let mut state = create_state();
    let mut errors = Vec::new();

    reduce(
        &mut state,
        ChoreographySettingsAction::InitializePositionsAtSide(false),
    );
    check!(errors, !state.positions_at_side);

    reduce(
        &mut state,
        ChoreographySettingsAction::UpdatePositionsAtSide(true),
    );

    check!(errors, state.positions_at_side);
    check!(errors, state.preferences.positions_at_side);
    check!(errors, state.choreography.settings.positions_at_side);
    check!(errors, state.redraw_requested);

    assert_no_errors(errors);
}
