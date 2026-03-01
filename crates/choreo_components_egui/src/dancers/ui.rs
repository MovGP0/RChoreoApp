use egui::Ui;

use super::actions::DancersAction;
use super::state::DancersState;

pub fn draw(ui: &mut Ui, state: &DancersState) -> Vec<DancersAction> {
    let mut actions: Vec<DancersAction> = Vec::new();
    ui.heading("dancers (egui scaffold)");
    ui.label(format!("flags: {}", state.flags.len()));
    if ui.button("Initialize").clicked() {
        actions.push(DancersAction::Initialize);
    }
    actions
}
