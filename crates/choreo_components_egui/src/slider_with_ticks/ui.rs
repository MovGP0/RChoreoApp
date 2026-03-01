use egui::Ui;

use super::actions::SliderWithTicksAction;
use super::state::SliderWithTicksState;

pub fn draw(ui: &mut Ui, state: &SliderWithTicksState) -> Vec<SliderWithTicksAction> {
    let mut actions: Vec<SliderWithTicksAction> = Vec::new();
    ui.heading("Slider With Ticks");

    let mut value = state.value;
    let slider = egui::Slider::new(&mut value, state.minimum..=state.maximum).show_value(true);
    if ui.add_enabled(state.is_enabled, slider).changed() {
        actions.push(SliderWithTicksAction::SetValue { value });
    }

    if !state.tick_values.is_empty() {
        ui.horizontal_wrapped(|ui| {
            ui.label("Ticks:");
            for tick in &state.tick_values {
                if let Some(tick_color) = state.tick_color {
                    ui.colored_label(tick_color, format!("{tick:.2}"));
                } else {
                    ui.label(format!("{tick:.2}"));
                }
            }
        });
    }

    if ui.button("Initialize").clicked() {
        actions.push(SliderWithTicksAction::Initialize);
    }

    actions
}
