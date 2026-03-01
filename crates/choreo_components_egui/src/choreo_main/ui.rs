use egui::Ui;

use super::actions::ChoreoMainAction;
use super::state::ChoreoMainState;

pub fn draw(ui: &mut Ui, state: &ChoreoMainState) -> Vec<ChoreoMainAction> {
    let mut actions: Vec<ChoreoMainAction> = Vec::new();
    ui.heading("choreo_main (egui scaffold)");
    ui.label(format!("flags: {}", state.flags.len()));
    if ui.button("Initialize").clicked() {
        actions.push(ChoreoMainAction::Initialize);
    }
    actions
}
