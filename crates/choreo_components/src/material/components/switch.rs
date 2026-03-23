use std::borrow::Cow;

use egui::Image;
use egui::Response;
use egui::Sense;
use egui::Stroke;
use egui::StrokeKind;
use egui::Ui;
use egui::vec2;

use crate::material::components::StateLayerStyle;
use crate::material::components::apply_tooltip;
use crate::material::components::icon::MaterialIconStyle;
use crate::material::components::icon::icon_with_style;
use crate::material::components::paint_state_layer_for_response;
use crate::material::styling::material_palette::material_palette_for_visuals;
use crate::material::styling::material_style_metrics::material_style_metrics;

pub struct SwitchResponse {
    pub response: Response,
    pub checked: bool,
    pub changed: bool,
}

pub struct Switch<'a> {
    pub checked: bool,
    pub enabled: bool,
    pub tooltip: Cow<'a, str>,
    pub on_icon: Option<Image<'static>>,
    pub off_icon: Option<Image<'static>>,
}

impl<'a> Switch<'a> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            checked: false,
            enabled: true,
            tooltip: Cow::Borrowed(""),
            on_icon: None,
            off_icon: None,
        }
    }

    pub fn show(self, ui: &mut Ui) -> SwitchResponse {
        let palette = material_palette_for_visuals(ui.visuals());
        let metrics = material_style_metrics();
        let desired = vec2(metrics.sizes.size_52, metrics.sizes.size_32);
        let (rect, mut response) = ui.allocate_exact_size(desired, Sense::click());
        let keyboard_toggle = self.enabled
            && response.has_focus()
            && ui.input(|input| {
                input.key_pressed(egui::Key::Enter) || input.key_pressed(egui::Key::Space)
            });
        let changed = self.enabled && (response.clicked() || keyboard_toggle);
        if response.clicked() && self.enabled {
            response.request_focus();
        }
        if keyboard_toggle {
            response.mark_changed();
        }
        let checked = if changed { !self.checked } else { self.checked };
        let has_icon = (checked && self.on_icon.is_some()) || self.off_icon.is_some();

        let track_fill = if !self.enabled {
            if checked {
                palette.on_surface.gamma_multiply(0.12)
            } else {
                palette.surface_variant.gamma_multiply(0.12)
            }
        } else if checked {
            palette.primary
        } else {
            palette.surface_container_highest
        };
        let track_stroke = if checked {
            Stroke::NONE
        } else if self.enabled {
            Stroke::new(2.0, palette.outline)
        } else {
            Stroke::new(2.0, palette.on_surface.gamma_multiply(0.12))
        };
        ui.painter().rect(
            rect,
            egui::CornerRadius::same((rect.height() * 0.5).round() as u8),
            track_fill,
            track_stroke,
            StrokeKind::Middle,
        );

        let indicator_padding = if checked || has_icon {
            metrics.paddings.padding_4
        } else {
            metrics.paddings.padding_8
        };
        let mut indicator_size = rect.height() - 2.0 * indicator_padding;
        if response.is_pointer_button_down_on() {
            indicator_size = rect.height() - 2.0;
        }
        let indicator_x = if checked {
            rect.right() - indicator_padding - indicator_size
        } else {
            rect.left() + indicator_padding
        };
        let indicator_rect = egui::Rect::from_min_size(
            egui::pos2(indicator_x, rect.center().y - indicator_size * 0.5),
            vec2(indicator_size, indicator_size),
        );
        let indicator_fill = if !self.enabled {
            if checked {
                palette.surface
            } else {
                palette.outline.gamma_multiply(0.38)
            }
        } else if response.is_pointer_button_down_on() {
            if checked {
                palette.primary_container
            } else {
                palette.on_surface_variant
            }
        } else if checked {
            palette.primary_container
        } else {
            palette.outline
        };
        ui.painter().circle_filled(
            indicator_rect.center(),
            indicator_rect.width() * 0.5,
            indicator_fill,
        );

        let icon = if checked {
            self.on_icon.clone()
        } else {
            self.off_icon.clone()
        };
        if let Some(icon) = icon {
            let foreground = if !self.enabled {
                if checked {
                    palette.on_surface.gamma_multiply(0.38)
                } else {
                    palette.surface_container_highest
                }
            } else if checked {
                palette.on_primary_container
            } else {
                palette.surface_container_highest
            };
            let _ = ui.put(
                indicator_rect,
                icon_with_style(
                    icon,
                    MaterialIconStyle {
                        size: vec2(
                            metrics.icon_sizes.icon_size_18,
                            metrics.icon_sizes.icon_size_18,
                        ),
                        tint: foreground,
                    },
                ),
            );
        }

        let mut state_style = StateLayerStyle::for_ui(ui);
        state_style.color = indicator_fill;
        state_style.border_radius = rect.height() * 0.5;
        state_style.enabled = self.enabled;
        state_style.tooltip = self.tooltip.as_ref();
        paint_state_layer_for_response(ui, &response, state_style);
        let response = apply_tooltip(ui, response, state_style);

        SwitchResponse {
            response,
            checked,
            changed,
        }
    }
}

impl Default for Switch<'_> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use egui::Context;
    use egui::Image;

    use super::Switch;

    #[test]
    fn switch_renders_without_panicking() {
        let context = Context::default();
        let mut size = 0.0;
        let _ = context.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let response = Switch::new().show(ui);
                size = response.response.rect.width();
            });
        });
        assert!(size >= 52.0);
    }

    #[test]
    fn switch_supports_icons() {
        let context = Context::default();
        let mut size = 0.0;
        let _ = context.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let response = Switch {
                    checked: true,
                    on_icon: Some(Image::new(egui::include_image!(
                        "../../../assets/icons/Check.svg"
                    ))),
                    ..Switch::new()
                }
                .show(ui);
                size = response.response.rect.height();
            });
        });
        assert!(size >= 32.0);
    }
}
