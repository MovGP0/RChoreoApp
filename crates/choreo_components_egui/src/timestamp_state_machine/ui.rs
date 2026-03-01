use egui::Ui;

use super::actions::TimestampStateMachineAction;
use super::state::TimestampStateMachineState;

pub fn draw(ui: &mut Ui, state: &TimestampStateMachineState) -> Vec<TimestampStateMachineAction> {
    let mut actions: Vec<TimestampStateMachineAction> = Vec::new();
    ui.heading("Timestamp Ownership");
    ui.label(format!("Playback: {:?}", state.playback_phase));
    ui.label(format!("Ownership: {:?}", state.ownership_phase));
    ui.label(format!(
        "Pending seek: {}",
        state
            .pending_seek_position
            .map(|value| value.to_string())
            .unwrap_or_else(|| "none".to_string())
    ));

    if ui.button("Reset").clicked() {
        actions.push(TimestampStateMachineAction::Initialize);
    }
    actions
}
