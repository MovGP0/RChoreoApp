use choreo_components_egui::choreography_settings::ChoreographySettingsViewModel;
use choreo_components_egui::choreography_settings::behaviors::UpdateCommentBehavior;
use choreo_components_egui::choreography_settings::messages::ChoreographySettingsCommand;

#[test]
fn view_model_dispatches_commands_through_reducer_contract() {
    let mut view_model = ChoreographySettingsViewModel::default();
    view_model.dispatch(ChoreographySettingsCommand::UpdateName("Parity".to_string()));
    view_model.dispatch(ChoreographySettingsCommand::UpdateComment("  test  ".to_string()));

    assert_eq!(view_model.state.name, "Parity");
    assert_eq!(view_model.state.comment, "test");
}

#[test]
fn update_behavior_wrappers_apply_command_flows() {
    let mut view_model = ChoreographySettingsViewModel::default();
    UpdateCommentBehavior::apply(&mut view_model, "  wrapped  ".to_string());
    assert_eq!(view_model.state.comment, "wrapped");
}
