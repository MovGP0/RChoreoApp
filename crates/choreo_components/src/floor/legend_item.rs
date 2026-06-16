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
    painter.rect_filled(
        legend_rect,
        tokens::LEGEND_SWATCH_RADIUS,
        palette.surface_container,
    );
    painter.rect_stroke(
        legend_rect,
        tokens::LEGEND_SWATCH_RADIUS,
        egui::Stroke::new(tokens::GRID_LINE_WIDTH, palette.outline_variant),
        egui::StrokeKind::Middle,
    );

    let square_x = legend_rect.left() + tokens::LEGEND_PADDING;
    let shortcut_x = square_x + tokens::LEGEND_SHORTCUT_OFFSET_X;
    let name_x = square_x + tokens::LEGEND_NAME_OFFSET_X;
    let position_x = legend_rect.right() - tokens::LEGEND_PADDING;
    let start_y = legend_rect.top() + tokens::LEGEND_PADDING;

    for (index, entry) in state.legend_entries.iter().enumerate() {
        let y = start_y + index as f32 * tokens::LEGEND_ROW_HEIGHT;
        let color = tokens::color32_from_rgba(entry.color);
        painter.circle_filled(
            egui::pos2(square_x, y + tokens::LEGEND_SWATCH_RADIUS),
            tokens::LEGEND_SWATCH_RADIUS,
            color,
        );
        if !entry.shortcut.trim().is_empty() {
            painter.text(
                egui::pos2(shortcut_x, y),
                egui::Align2::LEFT_TOP,
                &entry.shortcut,
                egui::TextStyle::Body.resolve(style),
                palette.on_surface,
            );
        }
        painter.text(
            egui::pos2(name_x, y),
            egui::Align2::LEFT_TOP,
            &entry.name,
            egui::TextStyle::Body.resolve(style),
            palette.on_surface,
        );
        if !entry.position_text.trim().is_empty() {
            painter.text(
                egui::pos2(position_x, y),
                egui::Align2::RIGHT_TOP,
                &entry.position_text,
                egui::TextStyle::Small.resolve(style),
                palette.on_surface_variant,
            );
        }
    }
}
