use egui::Ui;

use super::actions::ChoreographySettingsAction;
use super::state::ChoreographySettingsState;

pub fn draw(ui: &mut Ui, state: &ChoreographySettingsState) -> Vec<ChoreographySettingsAction> {
    let mut actions: Vec<ChoreographySettingsAction> = Vec::new();
    ui.heading("choreography_settings (egui scaffold)");
    ui.label(format!("flags: {}", state.flags.len()));
    if ui.button("Initialize").clicked() {
        actions.push(ChoreographySettingsAction::Initialize);
    }
    actions
}
