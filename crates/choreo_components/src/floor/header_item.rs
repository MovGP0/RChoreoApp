use egui::Rect;

use crate::material::styling::material_palette::MaterialPalette;

use super::geometry;
use super::state::FloorState;
use super::tokens;

pub(super) fn draw_header(
    painter: &egui::Painter,
    canvas_rect: Rect,
    state: &FloorState,
    style: &egui::Style,
    palette: MaterialPalette,
) {
    let Some(header_rect) = state.header_overlay_rect else {
        return;
    };

    let overlay = geometry::primitive_to_screen_rect(canvas_rect, header_rect);
    painter.rect_filled(overlay, 0.0, palette.surface_container_low);
    if !state.choreography_name.trim().is_empty() {
        painter.text(
            egui::pos2(
                overlay.center().x,
                overlay.top() + tokens::HEADER_TITLE_OFFSET_Y,
            ),
            egui::Align2::CENTER_TOP,
            &state.choreography_name,
            egui::TextStyle::Heading.resolve(style),
            palette.on_surface,
        );
    }
    if !state.scene_name.trim().is_empty() {
        painter.text(
            egui::pos2(
                overlay.center().x,
                overlay.top() + tokens::HEADER_SCENE_OFFSET_Y,
            ),
            egui::Align2::CENTER_TOP,
            &state.scene_name,
            egui::TextStyle::Body.resolve(style),
            palette.on_surface_variant,
        );
    }
}
