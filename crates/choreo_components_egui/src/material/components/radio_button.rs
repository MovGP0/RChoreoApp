use std::borrow::Cow;

use egui::Response;
use egui::Sense;
use egui::Ui;
use egui::vec2;

use crate::material::components::ListTile;
use crate::material::components::StateLayerStyle;
use crate::material::components::apply_tooltip;
use crate::material::components::paint_state_layer_for_response;
use crate::material::styling::material_palette::material_palette_for_visuals;
use crate::material::styling::material_style_metrics::material_style_metrics;

#[derive(Debug, Clone, PartialEq)]
pub struct RadioButton {
    pub checked: bool,
    pub enabled: bool,
    pub tooltip: String,
}

impl RadioButton {
    pub fn show(&mut self, ui: &mut Ui) -> Response {
        let palette = material_palette_for_visuals(ui.visuals());
        let metrics = material_style_metrics();
        let (rect, mut response) = ui.allocate_exact_size(
            vec2(metrics.sizes.size_40, metrics.sizes.size_40),
            Sense::click(),
        );
        let keyboard_activate = self.enabled
            && response.has_focus()
            && ui.input(|input| input.key_pressed(egui::Key::Enter) || input.key_pressed(egui::Key::Space));

        if response.clicked() && self.enabled {
            response.request_focus();
            self.checked = true;
        } else if keyboard_activate {
            response.mark_changed();
            self.checked = true;
        }

        let mut state_style = StateLayerStyle::for_ui(ui);
        state_style.color = palette.on_surface;
        state_style.border_radius = rect.width().max(rect.height()) * 0.5;
        state_style.enabled = self.enabled;
        state_style.tooltip = self.tooltip.as_str();
        paint_state_layer_for_response(ui, &response, state_style);
        let response = apply_tooltip(response, state_style);

        let border_rect =
            egui::Rect::from_center_size(rect.center(), vec2(metrics.sizes.size_20, metrics.sizes.size_20));
        let border_color = if !self.enabled {
            palette.on_surface.gamma_multiply(0.38)
        } else if self.checked {
            palette.primary
        } else if response.hovered() || response.has_focus() || response.is_pointer_button_down_on() {
            palette.on_surface
        } else {
            palette.on_surface_variant
        };
        ui.painter().circle_stroke(
            border_rect.center(),
            border_rect.width() * 0.5 - metrics.sizes.size_2 * 0.5,
            egui::Stroke::new(metrics.sizes.size_2, border_color),
        );

        let indicator_alpha = if self.checked {
            if self.enabled { 1.0 } else { 0.38 }
        } else {
            0.0
        };
        if indicator_alpha > 0.0 {
            let indicator_color = if self.enabled {
                palette.primary
            } else {
                palette.on_surface.gamma_multiply(0.38)
            };
            ui.painter().circle_filled(
                border_rect.center(),
                border_rect.width() * 0.25,
                indicator_color.gamma_multiply(indicator_alpha),
            );
        }

        response
    }
}

impl Default for RadioButton {
    fn default() -> Self {
        Self {
            checked: false,
            enabled: true,
            tooltip: String::new(),
        }
    }
}

pub struct RadioButtonTile<'a> {
    pub text: Cow<'a, str>,
    pub supporting_text: Cow<'a, str>,
    pub radio_button: RadioButton,
}

impl<'a> RadioButtonTile<'a> {
    #[must_use]
    pub fn new(text: impl Into<Cow<'a, str>>) -> Self {
        Self {
            text: text.into(),
            supporting_text: Cow::Borrowed(""),
            radio_button: RadioButton::default(),
        }
    }

    pub fn show(&mut self, ui: &mut Ui) -> Response {
        let text = self.text.clone();
        let supporting_text = self.supporting_text.clone();
        let radio_button = &mut self.radio_button;
        let response = ListTile {
            text,
            supporting_text,
            avatar_icon: None,
            avatar_text: Cow::Borrowed(""),
            avatar_background: None,
            avatar_foreground: None,
            enabled: radio_button.enabled,
            color: None,
            tooltip: Cow::Borrowed(""),
        }
        .show(ui, |ui| {
            let _ = radio_button.show(ui);
        });
        if response.clicked() && radio_button.enabled {
            radio_button.checked = true;
        }
        response
    }
}

#[cfg(test)]
mod tests {
    use egui::Context;

    use super::RadioButton;
    use super::RadioButtonTile;

    #[test]
    fn radio_button_renders_without_panicking() {
        let context = Context::default();
        let mut size = 0.0;
        let _ = context.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let mut radio = RadioButton::default();
                let response = radio.show(ui);
                size = response.rect.width().min(response.rect.height());
            });
        });
        assert!(size >= 40.0);
    }

    #[test]
    fn radio_button_tile_renders_without_panicking() {
        let context = Context::default();
        let mut height = 0.0;
        let _ = context.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let mut tile = RadioButtonTile::new("Option");
                let response = tile.show(ui);
                height = response.rect.height();
            });
        });
        assert!(height >= 72.0);
    }
}
