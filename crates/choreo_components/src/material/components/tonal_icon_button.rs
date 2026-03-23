use std::borrow::Cow;

use egui::Button;
use egui::CornerRadius;
use egui::Image;
use egui::Response;
use egui::Stroke;
use egui::Ui;
use egui::vec2;

use crate::material::components::centered_icon_rect;
use crate::material::components::paint_icon;
use crate::material::styling::material_palette::material_palette_for_visuals;
use crate::material::styling::material_style_metrics::material_style_metrics;

pub struct TonalIconButton<'a> {
    pub icon: Image<'static>,
    pub checked_icon: Option<Image<'static>>,
    pub checkable: bool,
    pub checked: bool,
    pub tooltip: Cow<'a, str>,
    pub enabled: bool,
}

impl<'a> TonalIconButton<'a> {
    #[must_use]
    pub fn new(icon: Image<'static>) -> Self {
        Self {
            icon,
            checked_icon: None,
            checkable: false,
            checked: false,
            tooltip: Cow::Borrowed(""),
            enabled: true,
        }
    }

    pub fn show(&mut self, ui: &mut Ui) -> Response {
        let palette = material_palette_for_visuals(ui.visuals());
        let metrics = material_style_metrics();
        let is_checked = self.enabled && self.checkable && self.checked;
        let fill = if self.enabled && self.checkable && !is_checked {
            palette.surface_container_highest
        } else if self.enabled {
            palette.secondary_container
        } else {
            palette.surface
        };
        let tint = if self.enabled {
            palette.on_secondary_container
        } else {
            palette.on_surface_variant
        };
        let icon = if is_checked {
            self.checked_icon
                .clone()
                .unwrap_or_else(|| self.icon.clone())
        } else {
            self.icon.clone()
        };
        let button = Button::new("")
            .fill(fill)
            .stroke(Stroke::NONE)
            .corner_radius(CornerRadius::same(
                (metrics.sizes.size_40 * 0.5).round() as u8
            ))
            .min_size(vec2(metrics.sizes.size_40, metrics.sizes.size_40));
        let response = ui
            .add_enabled(self.enabled, button)
            .on_hover_text(self.tooltip.clone().into_owned());
        paint_icon(
            ui,
            &icon,
            centered_icon_rect(
                response.rect,
                vec2(
                    metrics.icon_sizes.icon_size_24,
                    metrics.icon_sizes.icon_size_24,
                ),
            ),
            tint,
        );
        if response.clicked() && self.checkable {
            self.checked = !self.checked;
        }
        response
    }
}

#[cfg(test)]
mod tests {
    use egui::Context;
    use egui::Image;

    use super::TonalIconButton;

    #[test]
    fn tonal_icon_button_renders_without_panicking() {
        let context = Context::default();
        let mut width = 0.0;
        let _ = context.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let mut button = TonalIconButton::new(Image::new(egui::include_image!(
                    "../../../assets/icons/Home.svg"
                )));
                width = button.show(ui).rect.width();
            });
        });
        assert!(width >= 40.0);
    }
}
