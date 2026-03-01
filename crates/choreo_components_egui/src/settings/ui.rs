use egui::Ui;

use super::actions::SettingsAction;
use super::state::SettingsState;

pub fn draw(ui: &mut Ui, state: &SettingsState) -> Vec<SettingsAction> {
    let mut actions: Vec<SettingsAction> = Vec::new();
    ui.heading("settings (egui scaffold)");
    ui.label(format!("flags: {}", state.flags.len()));
    if ui.button("Initialize").clicked() {
        actions.push(SettingsAction::Initialize);
    }
    actions
}
