use egui::Ui;

use super::actions::FloorAction;
use super::state::FloorState;

pub fn draw(ui: &mut Ui, state: &FloorState) -> Vec<FloorAction> {
    let mut actions: Vec<FloorAction> = Vec::new();
    ui.heading("floor (egui scaffold)");
    ui.label(format!("flags: {}", state.flags.len()));
    if ui.button("Initialize").clicked() {
        actions.push(FloorAction::Initialize);
    }
    actions
}
