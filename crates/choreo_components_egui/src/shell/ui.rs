use egui::Ui;

use super::actions::ShellAction;
use super::state::ShellState;

pub fn draw(ui: &mut Ui, state: &ShellState) -> Vec<ShellAction> {
    let mut actions: Vec<ShellAction> = Vec::new();
    ui.heading("shell (egui scaffold)");
    ui.label(format!("flags: {}", state.flags.len()));
    if ui.button("Initialize").clicked() {
        actions.push(ShellAction::Initialize);
    }
    actions
}
