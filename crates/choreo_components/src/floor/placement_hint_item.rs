use egui::Rect;

use crate::material::styling::material_palette::MaterialPalette;

use super::state::FloorState;
use super::tokens;
use super::translations::floor_translations;

pub(super) fn draw_placement_hint(
    painter: &egui::Painter,
    canvas_rect: Rect,
    state: &FloorState,
    style: &egui::Style,
    palette: MaterialPalette,
) {
    let Some(remaining) = state.placement_remaining else {
        return;
    };
    if remaining == 0 {
        return;
    }

    let strings = floor_translations("en");
    let start = egui::pos2(
        canvas_rect.left() + tokens::PLACEMENT_TEXT_LEFT,
        canvas_rect.top() + tokens::PLACEMENT_TEXT_TOP,
    );
    painter.text(
        start,
        egui::Align2::LEFT_TOP,
        strings.placement_title,
        egui::TextStyle::Button.resolve(style),
        palette.on_surface,
    );
    painter.text(
        egui::pos2(start.x, start.y + tokens::PLACEMENT_TEXT_LINE_HEIGHT),
        egui::Align2::LEFT_TOP,
        strings.placement_hint,
        egui::TextStyle::Body.resolve(style),
        palette.on_surface_variant,
    );
    painter.text(
        egui::pos2(
            start.x,
            start.y + (tokens::PLACEMENT_TEXT_LINE_HEIGHT * 2.0),
        ),
        egui::Align2::LEFT_TOP,
        format!("{}{}", strings.placement_remaining_prefix, remaining),
        egui::TextStyle::Body.resolve(style),
        palette.secondary,
    );
}
