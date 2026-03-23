use egui::CornerRadius;
use egui::Response;
use egui::Sense;
use egui::Stroke;
use egui::Ui;
use egui::vec2;

use crate::styling::material_palette::material_palette_for_visuals;
use crate::styling::material_style_metrics::material_style_metrics;

pub struct SliderResponse {
    pub response: Response,
    pub value: f32,
    pub value_changed: bool,
    pub released: bool,
}

pub struct Slider {
    pub enabled: bool,
    pub value: f32,
    pub minimum: f32,
    pub maximum: f32,
    pub stop_count: i32,
}

impl Slider {
    #[must_use]
    pub fn new(value: f32, range: std::ops::RangeInclusive<f32>) -> Self {
        Self {
            enabled: true,
            value,
            minimum: *range.start(),
            maximum: *range.end(),
            stop_count: 0,
        }
    }

    pub fn show(&mut self, ui: &mut Ui) -> SliderResponse {
        let palette = material_palette_for_visuals(ui.visuals());
        let metrics = material_style_metrics();
        let desired_size = vec2(ui.available_width().max(metrics.sizes.size_40), 20.0);
        let (rect, mut response) = ui.allocate_exact_size(desired_size, Sense::click_and_drag());
        let steps = if self.stop_count > 0 {
            (self.maximum - self.minimum) / self.stop_count as f32
        } else {
            0.0
        };
        let keyboard_step = if steps > 0.0 {
            steps
        } else {
            ((self.maximum - self.minimum) / 100.0).max(f32::EPSILON)
        };

        let mut value_changed = false;
        if self.enabled
            && (response.clicked() || response.dragged())
            && let Some(pointer_pos) = response.interact_pointer_pos()
        {
            if response.clicked() {
                response.request_focus();
            }
            let normalized = ((pointer_pos.x - rect.left()) / rect.width()).clamp(0.0, 1.0);
            let raw_value = self.minimum + (self.maximum - self.minimum) * normalized;
            let new_value = if steps > 0.0 {
                ((raw_value - self.minimum) / steps).round() * steps + self.minimum
            } else {
                raw_value
            }
            .clamp(self.minimum, self.maximum);
            if (new_value - self.value).abs() > f32::EPSILON {
                self.value = new_value;
                value_changed = true;
            }
        }
        if self.enabled && response.has_focus() {
            let delta = ui.input(|input| {
                if input.key_pressed(egui::Key::ArrowLeft) {
                    Some(-keyboard_step)
                } else if input.key_pressed(egui::Key::ArrowRight) {
                    Some(keyboard_step)
                } else {
                    None
                }
            });
            if let Some(delta) = delta {
                let new_value = (self.value + delta).clamp(self.minimum, self.maximum);
                if (new_value - self.value).abs() > f32::EPSILON {
                    self.value = new_value;
                    value_changed = true;
                    response.mark_changed();
                }
            }
        }

        let released = response.drag_stopped() || response.clicked() || value_changed;

        let track_rect =
            egui::Rect::from_center_size(rect.center(), vec2(rect.width(), metrics.sizes.size_16));
        ui.painter().rect_filled(
            track_rect,
            CornerRadius::same((track_rect.height() * 0.5).round() as u8),
            palette.surface_container_highest,
        );

        let fraction = if (self.maximum - self.minimum).abs() <= f32::EPSILON {
            0.0
        } else {
            ((self.value - self.minimum) / (self.maximum - self.minimum)).clamp(0.0, 1.0)
        };
        let fill_rect = egui::Rect::from_min_max(
            track_rect.min,
            egui::pos2(
                track_rect.left() + track_rect.width() * fraction,
                track_rect.bottom(),
            ),
        );
        ui.painter().rect_filled(
            fill_rect,
            CornerRadius {
                nw: track_rect.height().round() as u8 / 2,
                sw: track_rect.height().round() as u8 / 2,
                ne: 1,
                se: 1,
            },
            palette.primary,
        );

        if self.stop_count > 1 && steps > 0.0 {
            let count = ((self.maximum - self.minimum) / steps).round() as i32;
            for stop in 1..count {
                let t = stop as f32 / count as f32;
                let x = track_rect.left() + track_rect.width() * t;
                ui.painter().circle_filled(
                    egui::pos2(x, track_rect.center().y),
                    metrics.sizes.size_4 * 0.5,
                    palette.outline,
                );
            }
        }

        let thumb_center = egui::pos2(fill_rect.right(), track_rect.center().y);
        ui.painter()
            .circle_filled(thumb_center, metrics.sizes.size_6, palette.primary);
        if response.hovered() || response.has_focus() || response.is_pointer_button_down_on() {
            ui.painter().circle_stroke(
                thumb_center,
                metrics.sizes.size_14,
                Stroke::new(
                    2.0,
                    palette
                        .primary
                        .gamma_multiply(if response.is_pointer_button_down_on() {
                            0.24
                        } else {
                            0.12
                        }),
                ),
            );
        }

        SliderResponse {
            response,
            value: self.value,
            value_changed,
            released,
        }
    }
}

#[cfg(test)]
mod tests {
    use egui::Context;

    use super::Slider;

    #[test]
    fn slider_renders_without_panicking() {
        let context = Context::default();
        let mut width = 0.0;
        let _ = context.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let mut slider = Slider::new(25.0, 0.0..=100.0);
                let response = slider.show(ui);
                width = response.response.rect.width();
            });
        });
        assert!(width > 0.0);
    }
}
