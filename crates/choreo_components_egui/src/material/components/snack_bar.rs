use std::borrow::Cow;

use egui::Image;
use egui::Response;
use egui::Sense;
use egui::Ui;
use egui::vec2;

use crate::material::components::ElevationSpec;
use crate::material::components::MaterialIconButton;
use crate::material::components::TextButton;
use crate::material::components::material_text;
use crate::material::components::paint_elevation;
use crate::material::styling::material_palette::material_palette_for_visuals;
use crate::material::styling::material_style_metrics::material_style_metrics;
use crate::material::styling::material_typography::MATERIAL_TYPOGRAPHY;

pub struct SnackBarResponse {
    pub response: Response,
    pub action_clicked: bool,
    pub close_clicked: bool,
}

pub struct SnackBar<'a> {
    pub text: Cow<'a, str>,
    pub action_text: Cow<'a, str>,
    pub has_close_button: bool,
    pub width: f32,
}

impl<'a> SnackBar<'a> {
    #[must_use]
    pub fn new(text: impl Into<Cow<'a, str>>) -> Self {
        Self {
            text: text.into(),
            action_text: Cow::Borrowed(""),
            has_close_button: false,
            width: material_style_metrics().sizes.size_344,
        }
    }

    pub fn show(self, ui: &mut Ui) -> SnackBarResponse {
        let palette = material_palette_for_visuals(ui.visuals());
        let metrics = material_style_metrics();
        let min_height = metrics.sizes.size_40 + metrics.paddings.padding_20;
        let (rect, response) =
            ui.allocate_exact_size(vec2(self.width, min_height), Sense::hover());
        paint_elevation(
            ui.painter(),
            rect,
            ElevationSpec {
                background: palette.inverse_surface,
                border_radius: metrics.corner_radii.border_radius_4,
                level: 3,
                dark_mode: ui.visuals().dark_mode,
            },
            palette,
        );
        let mut action_clicked = false;
        let mut close_clicked = false;
        ui.scope_builder(egui::UiBuilder::new().max_rect(rect), |ui| {
            ui.add_space(metrics.paddings.padding_10);
            ui.horizontal(|ui| {
                ui.add_space(metrics.paddings.padding_16);
                ui.set_max_width(self.width - metrics.paddings.padding_16 * 2.0);
                let _ = material_text(ui, self.text)
                    .text_style(MATERIAL_TYPOGRAPHY.body_medium)
                    .color(palette.inverse_on_surface)
                    .show(ui);
                if !self.action_text.is_empty() {
                    let response = TextButton {
                        text: self.action_text,
                        inverse: true,
                        ..TextButton::default()
                    }
                    .show(ui);
                    action_clicked = response.clicked();
                }
                if self.has_close_button {
                    let response = MaterialIconButton {
                        icon: Image::new(egui::include_image!("../../../assets/icons/Close.svg")),
                        inverse: true,
                        tooltip: Cow::Borrowed("Close"),
                        ..MaterialIconButton::new(Image::new(egui::include_image!(
                            "../../../assets/icons/Close.svg"
                        )))
                    }
                    .show(ui);
                    close_clicked = response.response.clicked();
                }
            });
        });
        SnackBarResponse {
            response,
            action_clicked,
            close_clicked,
        }
    }
}

impl Default for SnackBar<'_> {
    fn default() -> Self {
        Self::new("")
    }
}

#[cfg(test)]
mod tests {
    use egui::Context;

    use super::SnackBar;

    #[test]
    fn snack_bar_renders_without_panicking() {
        let context = Context::default();
        let mut width = 0.0;
        let _ = context.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let response = SnackBar::new("Saved").show(ui);
                width = response.response.rect.width();
            });
        });
        assert!(width >= 344.0);
    }
}
