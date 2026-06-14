use std::borrow::Cow;

use egui::Image;
use egui::Response;
use egui::Ui;

use crate::components::BaseButton;
use crate::styling::material_palette::MaterialPalette;
use crate::styling::material_palette::material_palette_for_visuals;
use crate::styling::material_style_metrics::material_style_metrics;

pub struct TextButton<'a> {
    pub icon: Option<Image<'static>>,
    pub text: Cow<'a, str>,
    pub tooltip: Cow<'a, str>,
    pub enabled: bool,
    pub inverse: bool,
}

impl<'a> TextButton<'a> {
    #[must_use]
    pub fn new(text: impl Into<Cow<'a, str>>) -> Self {
        Self {
            icon: None,
            text: text.into(),
            tooltip: Cow::Borrowed(""),
            enabled: true,
            inverse: false,
        }
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        let palette = material_palette_for_visuals(ui.visuals());
        BaseButton {
            icon: self.icon,
            text: self.text,
            tooltip: self.tooltip,
            enabled: self.enabled,
            color: Some(text_button_content_color(
                palette,
                self.enabled,
                self.inverse,
            )),
            border_radius: Some(material_style_metrics().sizes.size_40 * 0.5),
            display_background: false,
            ..BaseButton::new()
        }
        .show(ui, |_| {})
        .response
    }
}

impl Default for TextButton<'_> {
    fn default() -> Self {
        Self::new("")
    }
}

#[must_use]
pub fn text_button_content_color(
    palette: MaterialPalette,
    enabled: bool,
    inverse: bool,
) -> egui::Color32 {
    if !enabled {
        return palette.on_surface_variant;
    }

    if inverse {
        palette.inverse_primary
    } else {
        palette.primary
    }
}

#[cfg(test)]
mod tests {
    use egui::Context;

    use super::TextButton;
    use super::text_button_content_color;

    #[test]
    fn text_button_renders_without_panicking() {
        let context = Context::default();
        let mut min_height = 0.0;
        let _ = context.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let response = TextButton::new("Text").show(ui);
                min_height = response.rect.height();
            });
        });
        assert!(min_height >= 40.0);
    }

    #[test]
    fn text_button_colors_match_material_action_tokens() {
        let palette = crate::styling::material_palette::MaterialPalette::dark();

        assert_eq!(
            text_button_content_color(palette, true, false),
            palette.primary
        );
        assert_eq!(
            text_button_content_color(palette, true, true),
            palette.inverse_primary
        );
        assert_eq!(
            text_button_content_color(palette, false, false),
            palette.on_surface_variant
        );
    }
}
