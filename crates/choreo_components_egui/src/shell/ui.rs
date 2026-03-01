use egui::Ui;

use super::actions::ShellAction;
use super::state::ShellState;
use super::state::ShellThemeMode;

pub fn draw(ui: &mut Ui, state: &ShellState) -> Vec<ShellAction> {
    let mut actions: Vec<ShellAction> = Vec::new();
    ui.heading(&state.app_title);
    ui.label(format!("background: {}", state.active_background_hex));
    let is_dark = matches!(state.theme_mode, ShellThemeMode::Dark);
    ui.label(format!("dark mode: {is_dark}"));

    if ui.button("Switch Theme").clicked() {
        actions.push(ShellAction::SetThemeMode { is_dark: !is_dark });
    }
    if ui.button("Initialize").clicked() {
        actions.push(ShellAction::Initialize);
    }
    actions
}
