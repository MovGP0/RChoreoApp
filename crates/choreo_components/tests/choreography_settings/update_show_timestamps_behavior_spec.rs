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
fn update_show_timestamps_initializes_updates_state_and_emits_event() {
    let mut state = create_state();
    let mut errors = Vec::new();

    reduce(
        &mut state,
        ChoreographySettingsAction::InitializeShowTimestamps(false),
    );
    check!(errors, !state.show_timestamps);

    reduce(
        &mut state,
        ChoreographySettingsAction::UpdateShowTimestamps(true),
    );

    check!(errors, state.show_timestamps);
    check!(errors, state.preferences.show_timestamps);
    check!(errors, state.choreography.settings.show_timestamps);
    check!(errors, state.redraw_requested);
    check_eq!(
        errors,
        state
            .last_show_timestamps_event
            .map(|event| event.is_enabled),
        Some(true),
    );

    state.set_scene_timestamp_parts(1, 2, 39);
    check!(
        errors,
        (state.scene_timestamp_seconds - 62.03).abs() < 0.0001
    );

    reduce(
        &mut state,
        ChoreographySettingsAction::ClearEphemeralOutputs,
    );

    check!(errors, !state.redraw_requested);
    check!(errors, state.last_show_timestamps_event.is_none());

    assert_no_errors(errors);
}
