use egui::Rect;

use crate::material::styling::material_palette::MaterialPalette;

use super::geometry;
use super::state::FloorState;
use super::state::Point;
use super::tokens;
use super::tokens::FloorCanvasColorRoles;

pub(super) fn draw_background(
    painter: &egui::Painter,
    canvas_rect: Rect,
    state: &FloorState,
    roles: FloorCanvasColorRoles,
) {
    if let Some(background) = state.background_rect {
        painter.rect_filled(
            geometry::primitive_to_screen_rect(canvas_rect, background),
            0.0,
            roles.canvas_background,
        );
    }

    let floor_rect = geometry::to_screen_rect(
        canvas_rect,
        state.floor_x,
        state.floor_y,
        state.floor_width,
        state.floor_height,
    );
    painter.rect_filled(
        floor_rect,
        0.0,
        tokens::color32_from_rgba(state.floor_color),
    );
}

pub(super) fn draw_grid(
    painter: &egui::Painter,
    canvas_rect: Rect,
    state: &FloorState,
    palette: MaterialPalette,
    roles: FloorCanvasColorRoles,
) {
    for segment in &state.grid_lines {
        painter.line_segment(
            [
                geometry::to_screen_point(canvas_rect, segment.from),
                geometry::to_screen_point(canvas_rect, segment.to),
            ],
            egui::Stroke::new(tokens::GRID_LINE_WIDTH, roles.grid),
        );
    }

    let floor_stroke = egui::Stroke::new(tokens::FLOOR_BORDER_WIDTH, roles.floor_border);
    for segment in &state.center_mark_segments {
        painter.line_segment(
            [
                geometry::to_screen_point(canvas_rect, segment.from),
                geometry::to_screen_point(canvas_rect, segment.to),
            ],
            floor_stroke,
        );
    }

    painter.circle_filled(
        geometry::to_screen_point(canvas_rect, Point::new(state.center_x, state.center_y)),
        tokens::CENTER_MARK_RADIUS,
        palette.secondary,
    );

    painter.rect_stroke(
        geometry::to_screen_rect(
            canvas_rect,
            state.floor_x,
            state.floor_y,
            state.floor_width,
            state.floor_height,
        ),
        0.0,
        floor_stroke,
        egui::StrokeKind::Middle,
    );
}

pub(super) fn draw_svg_overlay_bounds(
    ui: &egui::Ui,
    canvas_rect: Rect,
    state: &FloorState,
    _palette: MaterialPalette,
) {
    let Some(bounds) = state.svg_overlay_bounds else {
        return;
    };
    let Some(bytes) = state.svg_source_bytes.as_ref() else {
        return;
    };
    let Some(path) = state.svg_source_path.as_ref() else {
        return;
    };

    let image_rect = geometry::primitive_to_screen_rect(canvas_rect, bounds);
    egui::Image::from_bytes(svg_image_uri(path), bytes.clone())
        .alt_text("Floor SVG overlay")
        .paint_at(ui, image_rect);
}

fn svg_image_uri(path: &str) -> String {
    format!("bytes://floor_overlay/{}", path.replace('\\', "/"))
}
