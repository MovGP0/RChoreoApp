use egui::Color32;
use egui::Layout;
use egui::Ui;

use crate::material::styling::material_palette::material_palette_for_visuals;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SplashScreenUiState {
    pub splash_image_path: String,
}

impl Default for SplashScreenUiState {
    fn default() -> Self {
        Self {
            splash_image_path: "splash.svg".to_string(),
        }
    }
}

pub fn draw(ui: &mut Ui, state: &SplashScreenUiState) {
    let full_rect = ui.max_rect();
    let background = material_palette_for_visuals(ui.visuals()).background;
    ui.painter().rect_filled(full_rect, 0.0, background);

    ui.set_min_size(full_rect.size());
    ui.with_layout(
        Layout::centered_and_justified(egui::Direction::TopDown),
        |ui| {
            if state.splash_image_path.trim().is_empty() {
                ui.colored_label(Color32::from_gray(160), " ");
                return;
            }

            let image = egui::Image::from_uri(state.splash_image_path.clone())
                .max_size(full_rect.size())
                .maintain_aspect_ratio(true)
                .shrink_to_fit();
            ui.add(image);
        },
    );
}
