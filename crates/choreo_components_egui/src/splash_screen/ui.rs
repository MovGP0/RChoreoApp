use egui::Ui;

use super::actions::SplashScreenAction;
use super::state::SplashScreenState;

pub fn draw(ui: &mut Ui, state: &SplashScreenState) -> Vec<SplashScreenAction> {
    let mut actions: Vec<SplashScreenAction> = Vec::new();
    ui.heading("splash_screen (egui scaffold)");
    ui.label(format!("flags: {}", state.flags.len()));
    if ui.button("Initialize").clicked() {
        actions.push(SplashScreenAction::Initialize);
    }
    actions
}
