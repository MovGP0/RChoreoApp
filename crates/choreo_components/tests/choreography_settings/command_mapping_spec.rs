use super::actions::ChoreographySettingsAction;
use super::actions::UpdateSelectedSceneAction;
use super::actions::from_command;
use super::messages::ChoreographySettingsCommand;
use super::messages::UpdateSelectedSceneCommand;
use crate::choreography_settings::state::DateParts;

#[test]
fn maps_command_to_action_for_date_and_transparency() {
    let date_action = from_command(ChoreographySettingsCommand::UpdateDate(DateParts {
        year: 2026,
        month: 3,
        day: 2,
    }));
    assert_eq!(
        date_action,
        ChoreographySettingsAction::UpdateDate {
            year: 2026,
            month: 3,
            day: 2
        }
    );

    let transparency_action = from_command(ChoreographySettingsCommand::UpdateTransparency(0.5));
    assert_eq!(
        transparency_action,
        ChoreographySettingsAction::UpdateTransparency(0.5)
    );
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
