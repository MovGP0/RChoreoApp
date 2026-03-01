use egui::Ui;

use super::actions::ObservabilityAction;
use super::state::ObservabilityState;

pub fn draw(ui: &mut Ui, state: &ObservabilityState) -> Vec<ObservabilityAction> {
    let mut actions: Vec<ObservabilityAction> = Vec::new();
    ui.heading("observability (egui scaffold)");
    ui.label(format!("flags: {}", state.flags.len()));
    if ui.button("Initialize").clicked() {
        actions.push(ObservabilityAction::Initialize);
    }
    actions
}
