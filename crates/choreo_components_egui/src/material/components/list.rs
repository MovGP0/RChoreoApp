use std::borrow::Cow;

use egui::Color32;
use egui::Image;
use egui::Response;
use egui::Ui;
use egui::Vec2;
use egui::vec2;

use crate::material::components::material_text::MaterialTextOverflow;
use crate::material::components::material_text::material_text;
use crate::material::components::state_layer::StateLayerStyle;
use crate::material::components::state_layer::apply_tooltip;
use crate::material::components::state_layer::paint_state_layer_for_response;
use crate::material::styling::material_palette::material_palette_for_visuals;
use crate::material::styling::material_style_metrics::material_style_metrics;
use crate::material::styling::material_typography::MATERIAL_TYPOGRAPHY;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AvatarStyle {
    pub background: Color32,
    pub size: Vec2,
}

impl AvatarStyle {
    #[must_use]
    pub fn for_ui(ui: &Ui) -> Self {
        let palette = material_palette_for_visuals(ui.visuals());
        let metrics = material_style_metrics();
        Self {
            background: palette.primary,
            size: vec2(metrics.sizes.size_32, metrics.sizes.size_32),
        }
    }
}

pub fn avatar(ui: &mut Ui, image: Option<Image<'static>>, style: AvatarStyle) -> Response {
    let (rect, response) = ui.allocate_exact_size(style.size, egui::Sense::hover());
    ui.painter().circle_filled(
        rect.center(),
        rect.width().min(rect.height()) * 0.5,
        style.background,
    );
    if let Some(image) = image {
        let image = image.fit_to_exact_size(style.size);
        ui.put(rect, image);
    }
    response
}

pub struct ListTile<'a> {
    pub text: Cow<'a, str>,
    pub supporting_text: Cow<'a, str>,
    pub avatar_icon: Option<Image<'static>>,
    pub avatar_text: Cow<'a, str>,
    pub avatar_background: Option<Color32>,
    pub avatar_foreground: Option<Color32>,
    pub enabled: bool,
    pub color: Option<Color32>,
    pub tooltip: Cow<'a, str>,
}

impl<'a> ListTile<'a> {
    #[must_use]
    pub fn new(text: impl Into<Cow<'a, str>>) -> Self {
        Self {
            text: text.into(),
            supporting_text: Cow::Borrowed(""),
            avatar_icon: None,
            avatar_text: Cow::Borrowed(""),
            avatar_background: None,
            avatar_foreground: None,
            enabled: true,
            color: None,
            tooltip: Cow::Borrowed(""),
        }
    }

    pub fn show(
        self,
        ui: &mut Ui,
        add_trailing: impl FnOnce(&mut Ui),
    ) -> Response {
        let palette = material_palette_for_visuals(ui.visuals());
        let metrics = material_style_metrics();
        let min_height = metrics.sizes.size_72;
        let color = self.color.unwrap_or(palette.on_surface);
        let inner = ui.allocate_ui_with_layout(
            vec2(ui.available_width().max(0.0), min_height),
            egui::Layout::left_to_right(egui::Align::Center),
            |ui| {
                ui.spacing_mut().item_spacing.x = metrics.spacings.spacing_16;
                ui.add_space(metrics.paddings.padding_16);

                if self.avatar_background.is_some()
                    || !self.avatar_text.is_empty()
                    || self.avatar_icon.is_some()
                {
                    let avatar_size = vec2(min_height - metrics.paddings.padding_16, min_height - metrics.paddings.padding_16);
                    let avatar_style = AvatarStyle {
                        background: self.avatar_background.unwrap_or(palette.primary),
                        size: avatar_size,
                    };
                    let _ = avatar(ui, self.avatar_icon, avatar_style);
                    if !self.avatar_text.is_empty() {
                        let rect = ui.min_rect();
                        ui.painter().text(
                            rect.center(),
                            egui::Align2::CENTER_CENTER,
                            self.avatar_text,
                            egui::FontId::proportional(MATERIAL_TYPOGRAPHY.title_medium.font_size_px),
                            self.avatar_foreground.unwrap_or(palette.on_primary),
                        );
                    }
                }

                ui.vertical(|ui| {
                    let _ = material_text(ui, self.text)
                        .color(color)
                        .text_style(MATERIAL_TYPOGRAPHY.body_large)
                        .overflow(MaterialTextOverflow::Elide)
                        .show(ui);
                    if !self.supporting_text.is_empty() {
                        let _ = material_text(ui, self.supporting_text)
                            .color(color)
                            .text_style(MATERIAL_TYPOGRAPHY.body_medium)
                            .overflow(MaterialTextOverflow::Elide)
                            .show(ui);
                    }
                });

                ui.add_space(metrics.paddings.padding_24);
                add_trailing(ui);
            },
        );

        let mut response = inner.response;
        let mut state_style = StateLayerStyle::for_ui(ui);
        state_style.color = color;
        state_style.enabled = self.enabled;
        state_style.tooltip = self.tooltip.as_ref();
        if !self.enabled {
            state_style.display_background = false;
        }
        paint_state_layer_for_response(ui, &response, state_style);
        response = apply_tooltip(response, state_style);
        response
    }
}

#[cfg(test)]
mod tests {
    use egui::Context;

    use super::ListTile;

    #[test]
    fn list_tile_renders_without_panicking() {
        let context = Context::default();
        let mut positive = false;
        let _ = context.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let response = ListTile::new("Example").show(ui, |_| {});
                positive = response.rect.height() > 0.0;
            });
        });
        assert!(positive);
    }
}
