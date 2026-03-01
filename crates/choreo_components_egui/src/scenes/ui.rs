use egui::Ui;

use super::actions::ScenesAction;
use super::state::ScenesState;

pub fn draw(ui: &mut Ui, state: &ScenesState) -> Vec<ScenesAction> {
    let mut actions: Vec<ScenesAction> = Vec::new();
    ui.heading("scenes (egui scaffold)");
    ui.label(format!("flags: {}", state.flags.len()));
    if ui.button("Initialize").clicked() {
        actions.push(ScenesAction::Initialize);
    }
    actions
}
