use egui::Ui;

use super::actions::HapticsAction;
use super::state::HapticsState;

pub fn draw(ui: &mut Ui, state: &HapticsState) -> Vec<HapticsAction> {
    let mut actions: Vec<HapticsAction> = Vec::new();
    ui.heading("Haptics");
    ui.label(format!("Backend: {:?}", state.backend));
    ui.label(format!("Supported: {}", state.supported));
    ui.label(format!("Delivered clicks: {}", state.delivered_count));

    if ui.button("Reinitialize").clicked() {
        actions.push(HapticsAction::Initialize);
    }
    if ui
        .add_enabled(state.supported, egui::Button::new("Test click"))
        .clicked()
    {
        actions.push(HapticsAction::TriggerClick);
    }
    actions
}
