use choreo_components::choreography_settings::actions::ChoreographySettingsAction;
use choreo_components::choreography_settings::actions::from_command;
use choreo_components::choreography_settings::messages::ChoreographySettingsCommand;
use choreo_components::choreography_settings::reducer::reduce;
use choreo_components::choreography_settings::state::ChoreographySettingsState;

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
fn commands_are_mapped_and_reduced_through_current_contract() {
    let mut state = ChoreographySettingsState::default();
    let update_name = from_command(ChoreographySettingsCommand::UpdateName(
        "Parity".to_string(),
    ));
    let update_comment = from_command(ChoreographySettingsCommand::UpdateComment(
        "  test  ".to_string(),
    ));
    reduce(&mut state, update_name);
    reduce(&mut state, update_comment);

    let mut errors = Vec::new();
    check_eq!(errors, state.name, "Parity");
    check_eq!(errors, state.comment, "test");
    assert_no_errors(errors);
}

#[test]
fn direct_action_reduction_updates_state() {
    let mut state = ChoreographySettingsState::default();
    reduce(
        &mut state,
        ChoreographySettingsAction::UpdateComment("  wrapped  ".to_string()),
    );
    assert_eq!(state.comment, "wrapped");
}
