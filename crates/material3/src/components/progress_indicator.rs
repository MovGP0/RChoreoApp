use egui::CornerRadius;
use egui::Response;
use egui::Sense;
use egui::Shape;
use egui::Stroke;
use egui::Ui;
use egui::pos2;
use egui::vec2;

use crate::styling::material_palette::material_palette_for_visuals;
use crate::styling::material_style_metrics::material_style_metrics;

pub struct LinearProgressIndicator {
    pub progress: f32,
    pub indeterminate: bool,
}

impl LinearProgressIndicator {
    #[must_use]
    pub fn new(progress: f32) -> Self {
        Self {
            progress,
            indeterminate: false,
        }
    }

    pub fn show(self, ui: &mut Ui, desired_width: f32) -> Response {
        let palette = material_palette_for_visuals(ui.visuals());
        let height = material_style_metrics().sizes.size_4;
        let (rect, response) =
            ui.allocate_exact_size(vec2(desired_width.max(0.0), height), Sense::hover());
        ui.painter()
            .rect_filled(rect, CornerRadius::same(2), palette.primary_container);
        let progress = if self.indeterminate {
            0.5
        } else {
            self.progress.clamp(0.0, 1.0)
        };
        let fill_rect = egui::Rect::from_min_max(
            rect.min,
            pos2(rect.min.x + rect.width() * progress, rect.max.y),
        );
        ui.painter()
            .rect_filled(fill_rect, CornerRadius::same(2), palette.primary);
        response
    }
}

pub struct CircularProgressIndicator {
    pub progress: f32,
    pub indeterminate: bool,
    pub size: f32,
    pub bar_height: f32,
}

impl CircularProgressIndicator {
    #[must_use]
    pub fn new(progress: f32) -> Self {
        let metrics = material_style_metrics();
        Self {
            progress,
            indeterminate: false,
            size: metrics.sizes.size_40,
            bar_height: metrics.sizes.size_4,
        }
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        let palette = material_palette_for_visuals(ui.visuals());
        let (rect, response) = ui.allocate_exact_size(vec2(self.size, self.size), Sense::hover());
        let center = rect.center();
        let radius = rect.width().min(rect.height()) * 0.5 - self.bar_height * 0.5;
        let track_color = if self.progress >= 1.0 {
            palette.primary
        } else {
            palette.primary_container
        };
        ui.painter()
            .circle_stroke(center, radius, Stroke::new(self.bar_height, track_color));
        let progress = if self.indeterminate {
            0.5
        } else {
            self.progress.clamp(0.0, 1.0)
        };
        if progress > 0.0 && progress < 1.0 {
            let start = if self.indeterminate {
                (ui.input(|input| input.time) as f32).fract() * std::f32::consts::TAU
            } else {
                -std::f32::consts::FRAC_PI_2
            };
            let sweep = progress * std::f32::consts::TAU;
            let points = arc_points(center, radius, start, sweep, 48);
            ui.painter().add(Shape::line(
                points,
                Stroke::new(self.bar_height, palette.primary),
            ));
        }
        response
    }
}

fn arc_points(
    center: egui::Pos2,
    radius: f32,
    start: f32,
    sweep: f32,
    steps: usize,
) -> Vec<egui::Pos2> {
    let mut points = Vec::with_capacity(steps + 1);
    for step in 0..=steps {
        let t = step as f32 / steps as f32;
        let angle = start + sweep * t;
        points.push(pos2(
            center.x + radius * angle.cos(),
            center.y + radius * angle.sin(),
        ));
    }
    points
}

#[cfg(test)]
mod tests {
    use egui::Context;

    use super::CircularProgressIndicator;
    use super::LinearProgressIndicator;

    #[test]
    fn linear_progress_indicator_renders_without_panicking() {
        let context = Context::default();
        let mut width = 0.0;
        let _ = context.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let response = LinearProgressIndicator::new(0.5).show(ui, 120.0);
                width = response.rect.width();
            });
        });
        assert!(width >= 120.0);
    }

    #[test]
    fn circular_progress_indicator_renders_without_panicking() {
        let context = Context::default();
        let mut size = 0.0;
        let _ = context.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let response = CircularProgressIndicator::new(0.5).show(ui);
                size = response.rect.width().min(response.rect.height());
            });
        });
        assert!(size >= 40.0);
    }
}
