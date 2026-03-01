use egui::Ui;

use super::actions::BehaviorAction;
use super::state::BehaviorState;

pub fn draw(ui: &mut Ui, state: &BehaviorState) -> Vec<BehaviorAction> {
    let mut actions: Vec<BehaviorAction> = Vec::new();
    ui.heading("behavior (egui scaffold)");
    ui.label(format!("flags: {}", state.flags.len()));
    if ui.button("Initialize").clicked() {
        actions.push(BehaviorAction::Initialize);
    }
    actions
}
