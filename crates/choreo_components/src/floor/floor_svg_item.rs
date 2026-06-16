use egui::Rect;

use super::geometry;
use super::state::FloorState;

pub(super) fn draw_svg_overlay(ui: &egui::Ui, canvas_rect: Rect, state: &FloorState) {
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
