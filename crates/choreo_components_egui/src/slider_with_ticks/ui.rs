use egui::Ui;

use super::actions::SliderWithTicksAction;
use super::state::SliderWithTicksState;

pub fn draw(ui: &mut Ui, state: &SliderWithTicksState) -> Vec<SliderWithTicksAction> {
    let mut actions: Vec<SliderWithTicksAction> = Vec::new();
    ui.heading("slider_with_ticks (egui scaffold)");
    ui.label(format!("flags: {}", state.flags.len()));
    if ui.button("Initialize").clicked() {
        actions.push(SliderWithTicksAction::Initialize);
    }
    actions
}
