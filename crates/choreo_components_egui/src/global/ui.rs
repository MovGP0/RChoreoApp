use egui::Ui;

use super::actions::GlobalAction;
use super::state::GlobalState;

pub fn draw(ui: &mut Ui, state: &GlobalState) -> Vec<GlobalAction> {
    let mut actions: Vec<GlobalAction> = Vec::new();
    ui.heading("global (egui scaffold)");
    ui.label(format!("flags: {}", state.flags.len()));
    if ui.button("Initialize").clicked() {
        actions.push(GlobalAction::Initialize);
    }
    actions
}
