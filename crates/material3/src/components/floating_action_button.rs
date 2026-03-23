use std::borrow::Cow;

use egui::Button;
use egui::CornerRadius;
use egui::Image;
use egui::Response;
use egui::RichText;
use egui::Stroke;
use egui::Ui;
use egui::vec2;

use crate::styling::material_palette::material_palette_for_visuals;
use crate::styling::material_style_metrics::material_style_metrics;
use crate::styling::material_typography::MATERIAL_TYPOGRAPHY;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FabStyle {
    Small,
    Standard,
    Large,
}

pub struct FloatingActionButton<'a> {
    pub icon: Option<Image<'static>>,
    pub text: Cow<'a, str>,
    pub tooltip: Cow<'a, str>,
    pub style: FabStyle,
}

impl<'a> FloatingActionButton<'a> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            icon: None,
            text: Cow::Borrowed(""),
            tooltip: Cow::Borrowed(""),
            style: FabStyle::Standard,
        }
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        let palette = material_palette_for_visuals(ui.visuals());
        let metrics = material_style_metrics();
        let (size, radius, icon_size, _padding) = match self.style {
            FabStyle::Small => (
                metrics.sizes.size_40,
                metrics.corner_radii.border_radius_12,
                metrics.icon_sizes.icon_size_24,
                metrics.paddings.padding_10,
            ),
            FabStyle::Standard => (
                metrics.sizes.size_56,
                metrics.corner_radii.border_radius_16,
                metrics.icon_sizes.icon_size_24,
                metrics.paddings.padding_14,
            ),
            FabStyle::Large => (
                metrics.sizes.size_90,
                metrics.corner_radii.border_radius_28,
                metrics.icon_sizes.icon_size_36,
                metrics.paddings.padding_30,
            ),
        };
        let rich_text = RichText::new(self.text.into_owned())
            .color(palette.on_primary)
            .size(MATERIAL_TYPOGRAPHY.label_large.font_size_px);
        let mut button = match self.icon {
            Some(icon) if !rich_text.text().is_empty() => Button::image_and_text(
                icon.fit_to_exact_size(vec2(icon_size, icon_size))
                    .tint(palette.on_primary),
                rich_text,
            ),
            Some(icon) => Button::image(
                icon.fit_to_exact_size(vec2(icon_size, icon_size))
                    .tint(palette.on_primary),
            ),
            None => Button::new(rich_text),
        };
        button = button
            .fill(palette.primary)
            .stroke(Stroke::NONE)
            .corner_radius(CornerRadius::same(radius.round() as u8))
            .min_size(vec2(size, size));
        let response = ui.add(button);
        if self.tooltip.is_empty() {
            response
        } else {
            response.on_hover_text(self.tooltip.into_owned())
        }
    }
}

impl Default for FloatingActionButton<'_> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use egui::Context;
    use egui::Image;

    use super::FabStyle;
    use super::FloatingActionButton;

    #[test]
    fn floating_action_button_renders_without_panicking() {
        let context = Context::default();
        let mut width = 0.0;
        let _ = context.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                width = FloatingActionButton {
                    icon: Some(Image::new(egui::include_image!(
                        "../../assets/icons/Home.svg"
                    ))),
                    text: "Create".into(),
                    tooltip: "Create".into(),
                    style: FabStyle::Standard,
                }
                .show(ui)
                .rect
                .width();
            });
        });
        assert!(width >= 56.0);
    }
}
