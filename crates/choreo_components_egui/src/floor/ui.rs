use egui::Ui;

use super::actions::FloorAction;
use super::state::FloorState;

pub fn draw(ui: &mut Ui, state: &FloorState) -> Vec<FloorAction> {
    let mut actions: Vec<FloorAction> = Vec::new();
    ui.heading("Floor");
    ui.label(format!("positions: {}", state.positions.len()));
    ui.label(format!(
        "transform: scale {:.2}, pan ({:.1}, {:.1})",
        state.transformation_matrix.scale_x,
        state.transformation_matrix.trans_x,
        state.transformation_matrix.trans_y
    ));
    if ui.button("Init").clicked() {
        actions.push(FloorAction::Initialize);
    }
    if ui.button("Draw").clicked() {
        actions.push(FloorAction::DrawFloor);
    }
    if ui.button("Reset Viewport").clicked() {
        actions.push(FloorAction::ResetViewport);
    }
    actions
}
