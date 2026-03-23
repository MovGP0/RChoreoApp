use choreo_components::choreography_settings::actions::ChoreographySettingsAction;
use choreo_components::choreography_settings::actions::from_command;
use choreo_components::choreography_settings::messages::ChoreographySettingsCommand;
use choreo_components::choreography_settings::reducer::reduce;
use choreo_components::choreography_settings::state::ChoreographySettingsState;

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

    assert_eq!(state.name, "Parity");
    assert_eq!(state.comment, "test");
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
