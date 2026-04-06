use egui::Pos2;
use egui::Rect;
use egui::pos2;
use egui::vec2;

const SCENE_ROW_HEIGHT_PX: f32 = 50.0;
const SCENE_ROW_HEIGHT_WITH_TIMESTAMPS_PX: f32 = 62.0;
const SCENE_ROW_VERTICAL_GAP_PX: f32 = 4.0;
const SCENE_ROW_SWATCH_X_PX: f32 = 8.0;
const SCENE_ROW_SWATCH_Y_PX: f32 = 8.0;
const SCENE_ROW_SWATCH_SIZE_PX: f32 = 12.0;
const SCENE_ROW_TEXT_LEFT_PX: f32 = 26.0;
const SCENE_ROW_TITLE_Y_PX: f32 = 8.0;
const SCENE_ROW_TIMESTAMP_Y_PX: f32 = 30.0;
const SCENE_ROW_ACCENT_WIDTH_PX: f32 = 4.0;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SceneListItemLayout {
    pub content_rect: Rect,
    pub accent_rect: Rect,
    pub swatch_rect: Rect,
    pub title_position: Pos2,
    pub timestamp_position: Pos2,
}

#[must_use]
pub fn row_height_px(show_timestamps: bool) -> f32 {
    if show_timestamps {
        SCENE_ROW_HEIGHT_WITH_TIMESTAMPS_PX
    } else {
        SCENE_ROW_HEIGHT_PX
    }
}

#[must_use]
pub fn layout_for_row_rect(row_rect: Rect, show_timestamps: bool) -> SceneListItemLayout {
    let content_rect = row_rect.shrink2(vec2(0.0, SCENE_ROW_VERTICAL_GAP_PX));
    let swatch_rect = Rect::from_min_size(
        pos2(
            content_rect.left() + SCENE_ROW_SWATCH_X_PX,
            content_rect.top() + SCENE_ROW_SWATCH_Y_PX,
        ),
        vec2(SCENE_ROW_SWATCH_SIZE_PX, SCENE_ROW_SWATCH_SIZE_PX),
    );
    let accent_rect = Rect::from_min_size(
        content_rect.min,
        vec2(SCENE_ROW_ACCENT_WIDTH_PX, content_rect.height()),
    );
    let text_left = content_rect.left() + SCENE_ROW_TEXT_LEFT_PX;
    let timestamp_y = if show_timestamps {
        content_rect.top() + SCENE_ROW_TIMESTAMP_Y_PX
    } else {
        content_rect.bottom()
    };

    SceneListItemLayout {
        content_rect,
        accent_rect,
        swatch_rect,
        title_position: pos2(text_left, content_rect.top() + SCENE_ROW_TITLE_Y_PX),
        timestamp_position: pos2(text_left, timestamp_y),
    }
}
