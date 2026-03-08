use std::borrow::Cow;

use egui::CornerRadius;
use egui::Id;
use egui::Order;
use egui::Pos2;
use egui::Response;
use egui::Ui;
use egui::Vec2;
use egui::vec2;

use crate::material::components::MaterialTextOverflow;
use crate::material::components::material_text;
use crate::material::styling::material_palette::material_palette_for_visuals;
use crate::material::styling::material_typography::MATERIAL_TYPOGRAPHY;

pub struct ToolTip<'a> {
    pub text: Cow<'a, str>,
    pub max_width: f32,
    pub padding: Vec2,
}

impl<'a> ToolTip<'a> {
    #[must_use]
    pub fn new(text: impl Into<Cow<'a, str>>) -> Self {
        Self {
            text: text.into(),
            max_width: 200.0,
            padding: vec2(8.0, 4.0),
        }
    }

    pub fn show(self, ui: &mut Ui, anchor: Pos2) -> Response {
        let palette = material_palette_for_visuals(ui.visuals());
        let width = self.max_width;
        egui::Area::new(Id::new(("material_tooltip", anchor.x.to_bits(), anchor.y.to_bits())))
            .order(Order::Tooltip)
            .fixed_pos(anchor)
            .show(ui.ctx(), |ui| {
                egui::Frame::new()
                    .fill(palette.inverse_surface)
                    .corner_radius(CornerRadius::same(4))
                    .inner_margin(egui::Margin::symmetric(
                        self.padding.x.round() as i8,
                        self.padding.y.round() as i8,
                    ))
                    .show(ui, |ui| {
                        ui.set_max_width(width);
                        let _ = material_text(ui, self.text)
                            .text_style(MATERIAL_TYPOGRAPHY.body_small)
                            .color(palette.inverse_on_surface)
                            .overflow(MaterialTextOverflow::Wrap)
                            .show(ui);
                    })
                    .response
            })
            .inner
    }
}

impl Default for ToolTip<'_> {
    fn default() -> Self {
        Self::new("")
    }
}

#[cfg(test)]
mod tests {
    use egui::Context;
    use egui::pos2;

    use super::ToolTip;

    #[test]
    fn tooltip_renders_without_panicking() {
        let context = Context::default();
        let mut size = 0.0;
        let _ = context.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let response = ToolTip::new("Tooltip").show(ui, pos2(24.0, 24.0));
                size = response.rect.width().max(response.rect.height());
            });
        });
        assert!(size > 0.0);
    }
}
