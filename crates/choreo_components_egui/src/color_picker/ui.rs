use egui::Ui;

use super::actions::ColorPickerAction;
use super::state::ColorPickerState;

pub fn draw(ui: &mut Ui, state: &ColorPickerState) -> Vec<ColorPickerAction> {
    let mut actions: Vec<ColorPickerAction> = Vec::new();
    ui.heading("color_picker (egui scaffold)");
    ui.label(format!("flags: {}", state.flags.len()));
    if ui.button("Initialize").clicked() {
        actions.push(ColorPickerAction::Initialize);
    }
    actions
}
