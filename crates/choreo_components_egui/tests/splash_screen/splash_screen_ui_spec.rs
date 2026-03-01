use super::actions::SplashScreenAction;
use super::state::SplashScreenState;
use super::ui;

#[test]
fn splash_screen_ui_draws_without_panicking_and_emits_no_actions() {
    let state = SplashScreenState::new();
    let context = egui::Context::default();
    let mut emitted_actions: Vec<SplashScreenAction> = Vec::new();

    let _ = context.run(egui::RawInput::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            emitted_actions = ui::draw(ui, &state);
        });
    });

    assert!(emitted_actions.is_empty());
}
