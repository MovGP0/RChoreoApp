use egui::Rect;

use crate::material::styling::material_palette::MaterialPalette;

use super::geometry;
use super::state::FloorState;
use super::tokens;

pub(super) fn draw_legend(
    painter: &egui::Painter,
    canvas_rect: Rect,
    state: &FloorState,
    style: &egui::Style,
    palette: MaterialPalette,
) {
    let Some(legend_panel_rect) = state.legend_panel_rect else {
        return;
    };

    let legend_rect = geometry::primitive_to_screen_rect(canvas_rect, legend_panel_rect);
    let scale = legend_visual_scale(state);
    let stroke_width = tokens::GRID_LINE_WIDTH * scale;
    let corner_radius = tokens::LEGEND_SWATCH_RADIUS * scale;
    painter.rect_filled(legend_rect, corner_radius, palette.surface_container);
    painter.rect_stroke(
        legend_rect,
        corner_radius,
        egui::Stroke::new(stroke_width, palette.outline_variant),
        egui::StrokeKind::Middle,
    );

    let padding_left = (state.metrics.legend_content_padding_left
        * state.transformation_matrix.scale_x.max(0.1)) as f32;
    let padding_top = (state.metrics.legend_content_padding_top
        * state.transformation_matrix.scale_x.max(0.1)) as f32;
    let padding_right = (state.metrics.legend_content_padding_right
        * state.transformation_matrix.scale_x.max(0.1)) as f32;
    let row_height = tokens::LEGEND_ROW_HEIGHT * scale;
    let swatch_radius = tokens::LEGEND_SWATCH_RADIUS * scale;
    let shortcut_offset = tokens::LEGEND_SHORTCUT_OFFSET_X * scale;
    let name_offset = tokens::LEGEND_NAME_OFFSET_X * scale;
    let body_font = egui::FontId::proportional(egui::TextStyle::Body.resolve(style).size * scale);
    let small_font = egui::FontId::proportional(egui::TextStyle::Small.resolve(style).size * scale);

    let square_x = legend_rect.left() + padding_left;
    let shortcut_x = square_x + shortcut_offset;
    let name_x = square_x + name_offset;
    let position_x = legend_rect.right() - padding_right;
    let start_y = legend_rect.top() + padding_top;

    for (index, entry) in state.legend_entries.iter().enumerate() {
        let y = start_y + index as f32 * row_height;
        let color = tokens::color32_from_rgba(entry.color);
        painter.circle_filled(
            egui::pos2(square_x, y + swatch_radius),
            swatch_radius,
            color,
        );
        if !entry.shortcut.trim().is_empty() {
            painter.text(
                egui::pos2(shortcut_x, y),
                egui::Align2::LEFT_TOP,
                &entry.shortcut,
                body_font.clone(),
                palette.on_surface,
            );
        }
        painter.text(
            egui::pos2(name_x, y),
            egui::Align2::LEFT_TOP,
            &entry.name,
            body_font.clone(),
            palette.on_surface,
        );
        if !entry.position_text.trim().is_empty() {
            painter.text(
                egui::pos2(position_x, y),
                egui::Align2::RIGHT_TOP,
                &entry.position_text,
                small_font.clone(),
                palette.on_surface_variant,
            );
        }
    }
}

fn legend_visual_scale(state: &FloorState) -> f32 {
    (state.zoom * state.transformation_matrix.scale_x.max(0.1)) as f32
}
