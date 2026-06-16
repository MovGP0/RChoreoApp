use egui::Rect;

use crate::material::styling::material_palette::MaterialPalette;

use super::geometry;
use super::state::FloorState;

pub(super) fn draw_axis_labels(
    painter: &egui::Painter,
    canvas_rect: Rect,
    state: &FloorState,
    style: &egui::Style,
    palette: MaterialPalette,
) {
    let font = egui::FontId::proportional(
        egui::TextStyle::Button.resolve(style).size * floor_visual_scale(state),
    );
    for axis in &state.axis_labels {
        painter.text(
            geometry::to_screen_point(canvas_rect, axis.position),
            egui::Align2::CENTER_CENTER,
            &axis.text,
            font.clone(),
            palette.on_surface_variant,
        );
    }
}

fn floor_visual_scale(state: &FloorState) -> f32 {
    (state.zoom * state.transformation_matrix.scale_x.max(0.1)) as f32
}
