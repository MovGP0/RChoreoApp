#[test]
fn splash_screen_host_ui_draws_without_panicking() {
    let state = super::splash_screen_host::ui::SplashScreenUiState::default();
    let context = egui::Context::default();

    let _ = context.run(egui::RawInput::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            super::splash_screen_host::ui::draw(ui, &state);
        });
    });
}

#[test]
fn splash_screen_host_handles_empty_image_path() {
    let state = super::splash_screen_host::ui::SplashScreenUiState {
        splash_image_path: String::new(),
    };
    let context = egui::Context::default();

    let _ = context.run(egui::RawInput::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            super::splash_screen_host::ui::draw(ui, &state);
        });
    });
}
