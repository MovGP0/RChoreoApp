use egui::Rect;
use egui::epaint::PathShape;

use crate::material::styling::material_palette::MaterialPalette;

use super::geometry;
use super::state::FloorState;
use super::tokens;
use super::tokens::FloorCanvasColorRoles;

pub(super) fn draw_paths(
    painter: &egui::Painter,
    canvas_rect: Rect,
    state: &FloorState,
    palette: MaterialPalette,
    roles: FloorCanvasColorRoles,
) {
    let floor_stroke = egui::Stroke::new(tokens::FLOOR_BORDER_WIDTH, roles.floor_border);
    if state.colored_path_segments.is_empty() {
        for segment in &state.path_segments {
            painter.line_segment(
                [
                    geometry::to_screen_point(canvas_rect, segment.from),
                    geometry::to_screen_point(canvas_rect, segment.to),
                ],
                floor_stroke,
            );
        }
    } else {
        for segment in &state.colored_path_segments {
            painter.line_segment(
                [
                    geometry::to_screen_point(canvas_rect, segment.from),
                    geometry::to_screen_point(canvas_rect, segment.to),
                ],
                egui::Stroke::new(
                    tokens::FLOOR_BORDER_WIDTH,
                    tokens::color32_from_rgba(segment.color),
                ),
            );
        }
    }

    if state.colored_dashed_path_segments.is_empty() {
        for segment in &state.dashed_path_segments {
            painter.add(PathShape::line(
                vec![
                    geometry::to_screen_point(canvas_rect, segment.from),
                    geometry::to_screen_point(canvas_rect, segment.to),
                ],
                egui::Stroke::new(tokens::PATH_LINE_WIDTH, palette.secondary),
            ));
        }
    } else {
        for segment in &state.colored_dashed_path_segments {
            painter.add(PathShape::line(
                vec![
                    geometry::to_screen_point(canvas_rect, segment.from),
                    geometry::to_screen_point(canvas_rect, segment.to),
                ],
                egui::Stroke::new(
                    tokens::PATH_LINE_WIDTH,
                    tokens::color32_from_rgba(segment.color),
                ),
            ));
        }
    }
}
