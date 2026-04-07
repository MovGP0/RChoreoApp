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
fn load_settings_preferences_updates_settings_fields() {
    let mut state = create_state();

    reduce(
        &mut state,
        ChoreographySettingsAction::LoadSettingsPreferences {
            show_timestamps: false,
            positions_at_side: false,
            snap_to_grid: true,
        },
    );

    let mut errors = Vec::new();

    check!(errors, !state.show_timestamps);
    check!(errors, !state.positions_at_side);
    check!(errors, state.snap_to_grid);
    check!(errors, !state.choreography.settings.show_timestamps);
    check!(errors, !state.choreography.settings.positions_at_side);
    check!(errors, state.choreography.settings.snap_to_grid);

    assert_no_errors(errors);
}
