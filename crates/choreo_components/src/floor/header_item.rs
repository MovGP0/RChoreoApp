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
    let visual_scale = geometry::floor_visual_scale(state);
    let title_font =
        egui::FontId::proportional(egui::TextStyle::Heading.resolve(style).size * visual_scale);
    let scene_font =
        egui::FontId::proportional(egui::TextStyle::Body.resolve(style).size * visual_scale);
    painter.rect_filled(overlay, 0.0, palette.surface_container_low);
    if !state.choreography_name.trim().is_empty() {
        painter.text(
            egui::pos2(
                overlay.center().x,
                overlay.top() + (tokens::HEADER_TITLE_OFFSET_Y * visual_scale),
            ),
            egui::Align2::CENTER_TOP,
            &state.choreography_name,
            title_font,
            palette.on_surface,
        );
    }
    if !state.scene_name.trim().is_empty() {
        painter.text(
            egui::pos2(
                overlay.center().x,
                overlay.top() + (tokens::HEADER_SCENE_OFFSET_Y * visual_scale),
            ),
            egui::Align2::CENTER_TOP,
            &state.scene_name,
            scene_font,
            palette.on_surface_variant,
        );
    }
}
