use egui::Ui;

use super::actions::DancersAction;
use super::state::DancersState;

pub fn draw(ui: &mut Ui, state: &DancersState) -> Vec<DancersAction> {
    let mut actions: Vec<DancersAction> = Vec::new();
    ui.heading("Dancers");
    ui.horizontal(|ui| {
        if ui.button("Reload").clicked() {
            actions.push(DancersAction::ReloadFromGlobal);
        }
        if ui.button("Add").clicked() {
            actions.push(DancersAction::AddDancer);
        }
        if ui
            .add_enabled(state.can_delete_dancer, egui::Button::new("Delete"))
            .clicked()
        {
            actions.push(DancersAction::DeleteSelectedDancer);
        }
        if ui
            .add_enabled(state.can_swap_dancers, egui::Button::new("Swap"))
            .clicked()
        {
            actions.push(DancersAction::SwapDancers);
        }
    });

    ui.separator();
    for index in 0..state.dancers.len() {
        let dancer = &state.dancers[index];
        let selected = state
            .selected_dancer
            .as_ref()
            .map(|value| value.dancer_id == dancer.dancer_id)
            .unwrap_or(false);
        let label = format!(
            "#{} {} ({})",
            dancer.dancer_id, dancer.name, dancer.role.name
        );
        if ui.selectable_label(selected, label).clicked() {
            actions.push(DancersAction::SelectDancer { index });
        }
    }

    actions
}
