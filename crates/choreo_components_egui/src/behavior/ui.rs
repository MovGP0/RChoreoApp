use egui::Ui;

use super::actions::BehaviorAction;
use super::state::BehaviorState;

pub fn draw(ui: &mut Ui, state: &BehaviorState) -> Vec<BehaviorAction> {
    let mut actions: Vec<BehaviorAction> = Vec::new();
    ui.heading("Behavior Lifecycle");
    ui.label(format!("registered disposables: {}", state.disposable_count()));
    if ui.button("Initialize").clicked() {
        actions.push(BehaviorAction::Initialize);
    }
    if ui.button("Dispose all").clicked() {
        actions.push(BehaviorAction::DisposeAll);
    }
    actions
}
