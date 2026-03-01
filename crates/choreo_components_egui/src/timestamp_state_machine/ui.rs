use egui::Ui;

use super::actions::TimestampStateMachineAction;
use super::state::TimestampStateMachineState;

pub fn draw(ui: &mut Ui, state: &TimestampStateMachineState) -> Vec<TimestampStateMachineAction> {
    let mut actions: Vec<TimestampStateMachineAction> = Vec::new();
    ui.heading("timestamp_state_machine (egui scaffold)");
    ui.label(format!("flags: {}", state.flags.len()));
    if ui.button("Initialize").clicked() {
        actions.push(TimestampStateMachineAction::Initialize);
    }
    actions
}
