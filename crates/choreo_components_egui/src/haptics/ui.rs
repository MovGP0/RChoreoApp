use egui::Ui;

use super::actions::HapticsAction;
use super::state::HapticsState;

pub fn draw(ui: &mut Ui, state: &HapticsState) -> Vec<HapticsAction> {
    let mut actions: Vec<HapticsAction> = Vec::new();
    ui.heading("haptics (egui scaffold)");
    ui.label(format!("flags: {}", state.flags.len()));
    if ui.button("Initialize").clicked() {
        actions.push(HapticsAction::Initialize);
    }
    actions
}
