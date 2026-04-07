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
fn update_draw_path_from_initializes_and_updates_flag() {
    let mut state = create_state();
    let mut errors = Vec::new();

    reduce(
        &mut state,
        ChoreographySettingsAction::InitializeDrawPathFrom(true),
    );
    check!(errors, state.draw_path_from);

    reduce(
        &mut state,
        ChoreographySettingsAction::UpdateDrawPathFrom(false),
    );

    check!(errors, !state.draw_path_from);
    check!(errors, !state.preferences.draw_path_from);
    check!(errors, state.redraw_requested);

    assert_no_errors(errors);
}
