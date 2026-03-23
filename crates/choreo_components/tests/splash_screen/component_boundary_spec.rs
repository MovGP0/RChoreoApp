#[test]
fn splash_screen_component_view_model_matches_original_minimal_contract() {
    let _view_model = super::splash_screen::SplashScreenViewModel::default();
}

#[test]
fn splash_screen_rendering_seam_lives_in_host_module() {
    let state = super::splash_screen_host::ui::SplashScreenUiState::default();
    let context = egui::Context::default();

    let _ = context.run(egui::RawInput::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            super::splash_screen_host::ui::draw(ui, &state);
        });
    });
}
