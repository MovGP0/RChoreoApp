use egui::Ui;

use super::actions::TimeAction;
use super::state::TimeState;

pub fn draw(ui: &mut Ui, state: &TimeState) -> Vec<TimeAction> {
    let mut actions: Vec<TimeAction> = Vec::new();
    ui.heading("time (egui scaffold)");
    ui.label(format!("flags: {}", state.flags.len()));
    if ui.button("Initialize").clicked() {
        actions.push(TimeAction::Initialize);
    }
    actions
}
