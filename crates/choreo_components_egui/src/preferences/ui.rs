use egui::Ui;

use super::actions::PreferencesAction;
use super::state::PreferencesState;

pub fn draw(ui: &mut Ui, state: &PreferencesState) -> Vec<PreferencesAction> {
    let mut actions: Vec<PreferencesAction> = Vec::new();
    ui.heading("preferences (egui scaffold)");
    ui.label(format!("flags: {}", state.flags.len()));
    if ui.button("Initialize").clicked() {
        actions.push(PreferencesAction::Initialize);
    }
    actions
}
