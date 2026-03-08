use egui::Rect;
use egui::Response;
use egui::Sense;
use egui::Ui;
use egui::Widget;
use egui::pos2;
use egui::vec2;

use crate::material::styling::material_palette::material_palette_for_visuals;

#[derive(Debug, Clone, Copy, Default)]
pub struct HorizontalDivider;

impl Widget for HorizontalDivider {
    fn ui(self, ui: &mut Ui) -> Response {
        let palette = material_palette_for_visuals(ui.visuals());
        let desired = vec2(ui.available_width().max(0.0), 1.0);
        let (rect, response) = ui.allocate_at_least(desired, Sense::hover());
        let line_rect = horizontal_divider_rect(rect.min.x, rect.min.y, rect.width());
        ui.painter().rect_filled(line_rect, 0.0, palette.outline_variant);
        response
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct VerticalDivider;

impl Widget for VerticalDivider {
    fn ui(self, ui: &mut Ui) -> Response {
        let palette = material_palette_for_visuals(ui.visuals());
        let desired = vec2(1.0, ui.available_height().max(0.0));
        let (rect, response) = ui.allocate_at_least(desired, Sense::hover());
        let line_rect = vertical_divider_rect(rect.min.x, rect.min.y, rect.height());
        ui.painter().rect_filled(line_rect, 0.0, palette.outline_variant);
        response
    }
}

pub fn draw_horizontal_divider(ui: &mut Ui) -> Response {
    ui.add(HorizontalDivider)
}

pub fn draw_vertical_divider(ui: &mut Ui) -> Response {
    ui.add(VerticalDivider)
}

#[must_use]
pub fn horizontal_divider_rect(origin_x: f32, origin_y: f32, width: f32) -> Rect {
    Rect::from_min_size(pos2(origin_x, origin_y), vec2(width.max(0.0), 1.0))
}

#[must_use]
pub fn vertical_divider_rect(origin_x: f32, origin_y: f32, height: f32) -> Rect {
    Rect::from_min_size(pos2(origin_x, origin_y), vec2(1.0, height.max(0.0)))
}

#[cfg(test)]
mod tests {
    use super::horizontal_divider_rect;
    use super::vertical_divider_rect;
    use super::HorizontalDivider;
    use super::VerticalDivider;
    use egui::Context;

    #[test]
    fn horizontal_divider_uses_one_pixel_height() {
        let rect = horizontal_divider_rect(10.0, 20.0, 120.0);
        assert_eq!(rect.width(), 120.0);
        assert_eq!(rect.height(), 1.0);
    }

    #[test]
    fn vertical_divider_uses_one_pixel_width() {
        let rect = vertical_divider_rect(10.0, 20.0, 96.0);
        assert_eq!(rect.width(), 1.0);
        assert_eq!(rect.height(), 96.0);
    }

    #[test]
    fn divider_long_axis_can_collapse_to_zero() {
        let horizontal = horizontal_divider_rect(10.0, 20.0, 0.0);
        let vertical = vertical_divider_rect(10.0, 20.0, 0.0);
        assert_eq!(horizontal.width(), 0.0);
        assert_eq!(horizontal.height(), 1.0);
        assert_eq!(vertical.width(), 1.0);
        assert_eq!(vertical.height(), 0.0);
    }

    #[test]
    fn widget_surfaces_can_render_without_explicit_long_axis_parameters() {
        let context = Context::default();
        let mut horizontal_positive = false;
        let mut vertical_positive = false;
        let _ = context.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                horizontal_positive = ui.add(HorizontalDivider).rect.height() == 1.0;
                vertical_positive = ui.add(VerticalDivider).rect.width() == 1.0;
            });
        });
        assert!(horizontal_positive);
        assert!(vertical_positive);
    }
}
