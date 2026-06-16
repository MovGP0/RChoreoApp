use crate::material::styling::material_palette::MaterialPalette;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FloorCanvasColorRoles {
    pub canvas_background: egui::Color32,
    pub grid: egui::Color32,
    pub floor_border: egui::Color32,
}

#[must_use]
pub fn floor_canvas_color_roles(palette: MaterialPalette) -> FloorCanvasColorRoles {
    FloorCanvasColorRoles {
        canvas_background: palette.surface,
        grid: palette.secondary,
        floor_border: palette.primary,
    }
}

#[must_use]
pub(super) fn color32_from_rgba(color: [u8; 4]) -> egui::Color32 {
    egui::Color32::from_rgba_unmultiplied(color[0], color[1], color[2], color[3])
}

pub(super) const FLOOR_BORDER_WIDTH: f32 = 2.0;
pub(super) const GRID_LINE_WIDTH: f32 = 1.0;
pub(super) const PATH_LINE_WIDTH: f32 = 1.0;
pub(super) const SELECTION_STROKE_WIDTH: f32 = 1.0;
pub(super) const DANCER_BORDER_WIDTH: f32 = 2.0;
pub(super) const DANCER_SELECTION_WIDTH: f32 = 3.0;

// Glyph radii are Material shape details, not layout grid dimensions.
pub(super) const CENTER_MARK_RADIUS: f32 = 4.0;
pub(super) const MIN_DANCER_RADIUS: f32 = 6.0;
pub(super) const DANCER_SELECTION_RADIUS_OFFSET: f32 = 4.0;
pub(super) const FALLBACK_POSITION_RADIUS: f32 = 6.0;
pub(super) const LEGEND_SWATCH_RADIUS: f32 = 6.0;

pub(super) const POSITION_LABEL_OFFSET_X: f64 = 12.0;
pub(super) const POSITION_LABEL_OFFSET_Y: f64 = -12.0;
pub(super) const HEADER_TITLE_OFFSET_Y: f32 = 12.0;
pub(super) const HEADER_SCENE_OFFSET_Y: f32 = 36.0;
pub(super) const LEGEND_PADDING: f32 = 12.0;
pub(super) const LEGEND_ROW_HEIGHT: f32 = 24.0;
pub(super) const LEGEND_SHORTCUT_OFFSET_X: f32 = 24.0;
pub(super) const LEGEND_NAME_OFFSET_X: f32 = 72.0;
pub(super) const PLACEMENT_TEXT_LEFT: f32 = 12.0;
pub(super) const PLACEMENT_TEXT_TOP: f32 = 12.0;
pub(super) const PLACEMENT_TEXT_LINE_HEIGHT: f32 = 24.0;
