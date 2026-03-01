use egui::Ui;

use super::actions::SplashScreenAction;
use super::state::SplashScreenState;

pub fn draw(ui: &mut Ui, state: &SplashScreenState) -> Vec<SplashScreenAction> {
    let full_rect = ui.max_rect();
    ui.painter()
        .rect_filled(full_rect, 0.0, state.background_color);

    ui.put(
        full_rect,
        egui::Image::from_uri(state.splash_image_path.clone()).max_size(full_rect.size()),
    );

    Vec::new()
}
