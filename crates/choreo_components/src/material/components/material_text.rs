use std::borrow::Cow;

use egui::Color32;
use egui::FontFamily;
use egui::FontId;
use egui::Label;
use egui::Response;
use egui::RichText;
use egui::TextWrapMode;
use egui::Ui;

use crate::material::styling::material_palette::material_palette_for_visuals;
use crate::material::styling::material_typography::MATERIAL_TYPOGRAPHY;
use crate::material::styling::material_typography::TextStyle;
use crate::material::styling::material_typography::apply_font_weight_to_rich_text;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaterialTextOverflow {
    Elide,
    Clip,
    Wrap,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MaterialTextStyle {
    pub style: TextStyle,
    pub color: Color32,
    pub overflow: MaterialTextOverflow,
}

impl MaterialTextStyle {
    #[must_use]
    pub fn for_ui(ui: &Ui) -> Self {
        let palette = material_palette_for_visuals(ui.visuals());
        Self {
            style: MATERIAL_TYPOGRAPHY.body_medium,
            color: palette.on_background,
            overflow: MaterialTextOverflow::Elide,
        }
    }
}

pub struct MaterialText<'a> {
    text: Cow<'a, str>,
    style: MaterialTextStyle,
}

impl<'a> MaterialText<'a> {
    #[must_use]
    pub fn new(ui: &Ui, text: impl Into<Cow<'a, str>>) -> Self {
        Self {
            text: text.into(),
            style: MaterialTextStyle::for_ui(ui),
        }
    }

    #[must_use]
    pub fn text_style(mut self, style: TextStyle) -> Self {
        self.style.style = style;
        self
    }

    #[must_use]
    pub fn color(mut self, color: Color32) -> Self {
        self.style.color = color;
        self
    }

    #[must_use]
    pub fn overflow(mut self, overflow: MaterialTextOverflow) -> Self {
        self.style.overflow = overflow;
        self
    }

    #[must_use]
    pub fn rich_text(&self) -> RichText {
        let rich_text = RichText::new(self.text.clone().into_owned())
            .font(FontId::new(
                self.style.style.font_size_px,
                FontFamily::Proportional,
            ))
            .color(self.style.color);
        apply_font_weight_to_rich_text(rich_text, self.style.style)
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        let label = match self.style.overflow {
            MaterialTextOverflow::Wrap => {
                Label::new(self.rich_text()).wrap_mode(TextWrapMode::Wrap)
            }
            MaterialTextOverflow::Clip => {
                Label::new(self.rich_text()).wrap_mode(TextWrapMode::Truncate)
            }
            MaterialTextOverflow::Elide => {
                Label::new(self.rich_text()).wrap_mode(TextWrapMode::Truncate)
            }
        };
        ui.add(label)
    }
}

#[must_use]
pub fn material_text<'a>(ui: &Ui, text: impl Into<Cow<'a, str>>) -> MaterialText<'a> {
    MaterialText::new(ui, text)
}

#[cfg(test)]
mod tests {
    use egui::Context;

    use super::MaterialTextOverflow;
    use super::material_text;
    use crate::material::styling::material_typography::MATERIAL_TYPOGRAPHY;

    #[test]
    fn defaults_match_slint_body_medium_surface() {
        let context = Context::default();
        let mut rendered_height = 0.0;
        let _ = context.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let response = material_text(ui, "hello").show(ui);
                rendered_height = response.rect.height();
            });
        });
        assert!(rendered_height >= MATERIAL_TYPOGRAPHY.body_medium.font_size_px);
    }

    #[test]
    fn wrap_mode_can_be_changed() {
        let context = Context::default();
        let mut did_render = false;
        let _ = context.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let response = material_text(ui, "hello")
                    .overflow(MaterialTextOverflow::Wrap)
                    .show(ui);
                did_render = response.rect.is_positive();
            });
        });
        assert!(did_render);
    }
}
