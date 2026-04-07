use super::actions::ChoreographySettingsAction;
use super::actions::UpdateSelectedSceneAction;
use super::actions::from_command;
use super::messages::ChoreographySettingsCommand;
use super::messages::UpdateSelectedSceneCommand;
use crate::choreography_settings::state::DateParts;

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
fn maps_command_to_action_for_date_and_transparency() {
    let mut errors = Vec::new();

    let date_action = from_command(ChoreographySettingsCommand::UpdateDate(DateParts {
        year: 2026,
        month: 3,
        day: 2,
    }));
    check_eq!(
        errors,
        date_action,
        ChoreographySettingsAction::UpdateDate {
            year: 2026,
            month: 3,
            day: 2
        }
    );

    let transparency_action = from_command(ChoreographySettingsCommand::UpdateTransparency(0.5));
    check_eq!(
        errors,
        transparency_action,
        ChoreographySettingsAction::UpdateTransparency(0.5)
    );

    assert_no_errors(errors);
}

#[test]
fn maps_selected_scene_command_to_action() {
    let action = from_command(ChoreographySettingsCommand::UpdateSelectedScene(
        UpdateSelectedSceneCommand::SceneTimestamp {
            has_timestamp: true,
            seconds: 12.34,
        },
    ));

    assert_eq!(
        action,
        ChoreographySettingsAction::UpdateSelectedScene(
            UpdateSelectedSceneAction::SceneTimestamp {
                has_timestamp: true,
                seconds: 12.34
            }
        )
    );
}
