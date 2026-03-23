use std::borrow::Cow;

use egui::Color32;
use egui::Image;
use egui::Response;
use egui::Sense;
use egui::Ui;
use egui::UiBuilder;
use egui::Vec2;
use egui::vec2;

use crate::components::BaseButton;
use crate::components::ElevationSpec;
use crate::components::paint_elevation;
use crate::styling::material_palette::material_palette_for_visuals;
use crate::styling::material_style_metrics::material_style_metrics;
use crate::styling::material_typography::MATERIAL_TYPOGRAPHY;

pub struct ElevatedButton<'a> {
    pub icon: Option<Image<'static>>,
    pub text: Cow<'a, str>,
    pub tooltip: Cow<'a, str>,
    pub enabled: bool,
}

impl<'a> ElevatedButton<'a> {
    #[must_use]
    pub fn new(text: impl Into<Cow<'a, str>>) -> Self {
        Self {
            icon: None,
            text: text.into(),
            tooltip: Cow::Borrowed(""),
            enabled: true,
        }
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        let palette = material_palette_for_visuals(ui.visuals());
        let base = BaseButton {
            icon: self.icon,
            text: self.text,
            tooltip: self.tooltip,
            enabled: self.enabled,
            color: Some(if self.enabled {
                palette.primary
            } else {
                palette.on_surface
            }),
            border_radius: Some(material_style_metrics().sizes.size_40 * 0.5),
            display_background: false,
            ..BaseButton::new()
        };
        let desired_size = desired_size(ui, &base);
        let border_radius = desired_size.y * 0.5;
        let (rect, shell_response) = ui.allocate_exact_size(desired_size, Sense::hover());
        if self.enabled {
            let level = if shell_response.hovered() { 3 } else { 1 };
            paint_elevation(
                ui.painter(),
                rect,
                ElevationSpec {
                    background: palette.surface_container_low,
                    border_radius,
                    level,
                    dark_mode: ui.visuals().dark_mode,
                },
                palette,
            );
        }
        ui.scope_builder(UiBuilder::new().max_rect(rect), |ui| {
            BaseButton {
                border_radius: Some(border_radius),
                ..base
            }
            .show(ui, |_| {})
            .response
        })
        .inner
    }
}

impl Default for ElevatedButton<'_> {
    fn default() -> Self {
        Self::new("")
    }
}

fn desired_size(ui: &Ui, button: &BaseButton<'_>) -> Vec2 {
    let font_height = ui.fonts(|fonts| {
        fonts.row_height(&egui::FontId::proportional(
            MATERIAL_TYPOGRAPHY.label_large.font_size_px,
        ))
    });
    let leading_width = if button.icon.is_some() {
        button.icon_size
    } else {
        0.0
    };
    let content_height = if button.text.is_empty() {
        leading_width
    } else {
        leading_width.max(font_height)
    } + button
        .button_padding_top
        .unwrap_or(button.button_vertical_padding)
        + button
            .button_padding_bottom
            .unwrap_or(button.button_vertical_padding);
    let text_width = if button.text.is_empty() {
        0.0
    } else {
        ui.fonts(|fonts| {
            fonts
                .layout_no_wrap(
                    button.text.clone().into_owned(),
                    egui::FontId::proportional(MATERIAL_TYPOGRAPHY.label_large.font_size_px),
                    Color32::WHITE,
                )
                .size()
                .x
        })
    };
    let padding_left = button
        .button_padding_left
        .unwrap_or(button.button_horizontal_padding);
    let padding_right = button
        .button_padding_right
        .unwrap_or(button.button_horizontal_padding);
    vec2(
        button.min_layout_width.max(
            padding_left
                + padding_right
                + leading_width
                + if leading_width > 0.0 && !button.text.is_empty() {
                    button.spacing
                } else {
                    0.0
                }
                + text_width,
        ),
        button.min_layout_height.max(content_height),
    )
}

#[cfg(test)]
mod tests {
    use egui::Context;

    use super::ElevatedButton;

    #[test]
    fn elevated_button_renders_without_panicking() {
        let context = Context::default();
        let mut min_height = 0.0;
        let _ = context.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let response = ElevatedButton::new("Elevated").show(ui);
                min_height = response.rect.height();
            });
        });
        assert!(min_height >= 40.0);
    }
}
