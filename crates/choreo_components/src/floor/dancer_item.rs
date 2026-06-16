use egui::Rect;

use crate::material::styling::material_palette::MaterialPalette;

use super::geometry;
use super::state::FloorState;
use super::state::Point;
use super::tokens;

pub(super) fn draw_position_circles(
    painter: &egui::Painter,
    canvas_rect: Rect,
    state: &FloorState,
    palette: MaterialPalette,
) {
    if state.rendered_positions.is_empty() {
        for point in &state.position_circles {
            painter.circle_filled(
                geometry::to_screen_point(canvas_rect, *point),
                tokens::FALLBACK_POSITION_RADIUS,
                palette.primary_container,
            );
        }

        return;
    }

    let radius = geometry::clamped_floor_position_radius(state);
    for position in &state.rendered_positions {
        let center = geometry::to_screen_point(canvas_rect, position.point);
        if position.is_selected {
            painter.circle_stroke(
                center,
                radius + tokens::DANCER_SELECTION_RADIUS_OFFSET,
                egui::Stroke::new(tokens::DANCER_SELECTION_WIDTH, palette.secondary),
            );
        }
        painter.circle_filled(
            center,
            radius,
            tokens::color32_from_rgba(position.fill_color),
        );
        painter.circle_stroke(
            center,
            radius,
            egui::Stroke::new(
                tokens::DANCER_BORDER_WIDTH,
                tokens::color32_from_rgba(position.border_color),
            ),
        );
    }
}

pub(super) fn draw_position_numbers(
    painter: &egui::Painter,
    canvas_rect: Rect,
    state: &FloorState,
    style: &egui::Style,
    palette: MaterialPalette,
) {
    if state.rendered_positions.is_empty() {
        for label in &state.position_labels {
            painter.text(
                geometry::to_screen_point(
                    canvas_rect,
                    Point::new(
                        label.point.x + tokens::POSITION_LABEL_OFFSET_X,
                        label.point.y + tokens::POSITION_LABEL_OFFSET_Y,
                    ),
                ),
                egui::Align2::LEFT_TOP,
                &label.text,
                egui::TextStyle::Body.resolve(style),
                palette.on_surface,
            );
        }

        return;
    }

    let radius = geometry::clamped_floor_position_radius(state);
    for position in &state.rendered_positions {
        if position.shortcut.trim().is_empty() {
            continue;
        }

        painter.text(
            geometry::to_screen_point(canvas_rect, position.point),
            egui::Align2::CENTER_CENTER,
            &position.shortcut,
            egui::FontId::proportional((radius * 1.15).max(12.0)),
            tokens::color32_from_rgba(position.text_color),
        );
    }
}
