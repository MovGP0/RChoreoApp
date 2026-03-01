use egui::Ui;

use super::actions::LoggingAction;
use super::state::LoggingState;

pub fn draw(ui: &mut Ui, state: &LoggingState) -> Vec<LoggingAction> {
    let mut actions: Vec<LoggingAction> = Vec::new();
    ui.heading("logging (egui scaffold)");
    ui.label(format!("flags: {}", state.flags.len()));
    if ui.button("Initialize").clicked() {
        actions.push(LoggingAction::Initialize);
    }
    actions
}
