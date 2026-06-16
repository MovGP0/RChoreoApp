use egui::Rect;

use crate::material::styling::material_palette::MaterialPalette;

use super::geometry;
use super::state::FloorState;
use super::tokens;

pub(super) fn draw_selection(
    painter: &egui::Painter,
    canvas_rect: Rect,
    state: &FloorState,
    palette: MaterialPalette,
) {
    for segment in &state.selection_segments {
        painter.line_segment(
            [
                geometry::to_screen_point(canvas_rect, segment.from),
                geometry::to_screen_point(canvas_rect, segment.to),
            ],
            egui::Stroke::new(tokens::SELECTION_STROKE_WIDTH, palette.secondary),
        );
    }
}
