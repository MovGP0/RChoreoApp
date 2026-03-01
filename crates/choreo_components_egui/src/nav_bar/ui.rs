use egui::Ui;

use super::actions::NavBarAction;
use super::state::NavBarState;

pub fn draw(ui: &mut Ui, state: &NavBarState) -> Vec<NavBarAction> {
    let mut actions: Vec<NavBarAction> = Vec::new();
    ui.heading("nav_bar (egui scaffold)");
    ui.label(format!("flags: {}", state.flags.len()));
    if ui.button("Initialize").clicked() {
        actions.push(NavBarAction::Initialize);
    }
    actions
}
