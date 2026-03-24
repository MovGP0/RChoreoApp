use egui::Color32;
use egui::Pos2;
use egui::Rect;
use egui::Vec2;
use egui::pos2;
use egui::vec2;

use crate::dancers::dancer_list_item_view::tokens::ROW_HEIGHT_PX;
use crate::dancers::dancer_list_item_view::tokens::item_top_bottom_gap_token;
use crate::dancers::dancer_list_item_view::tokens::subtitle_y_token;
use crate::dancers::dancer_list_item_view::tokens::swatch_half_height_token;
use crate::dancers::dancer_list_item_view::tokens::swatch_size_token;
use crate::dancers::dancer_list_item_view::tokens::swatch_x_token;
use crate::dancers::dancer_list_item_view::tokens::title_x_token;
use crate::dancers::dancer_list_item_view::tokens::title_y_token;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DancerListItemLayout {
    pub content_rect: Rect,
    pub swatch_rect: Rect,
    pub title_position: Pos2,
    pub subtitle_position: Pos2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DancerListItemColors {
    pub background: Color32,
    pub border: Color32,
    pub title: Color32,
    pub subtitle: Color32,
}

#[must_use]
pub fn layout_for_row_rect(row_rect: Rect) -> DancerListItemLayout {
    let content_rect = row_rect.shrink2(vec2(0.0, item_top_bottom_gap_token()));
    let swatch_rect = Rect::from_min_size(
        pos2(
            content_rect.left() + swatch_x_token(),
            content_rect.center().y - swatch_half_height_token(),
        ),
        vec2(swatch_size_token(), swatch_size_token()),
    );
    let text_left = content_rect.left() + title_x_token();

    DancerListItemLayout {
        content_rect,
        swatch_rect,
        title_position: pos2(text_left, content_rect.top() + title_y_token()),
        subtitle_position: pos2(text_left, content_rect.top() + subtitle_y_token()),
    }
}

#[must_use]
pub fn colors_for_selection(visuals: &egui::Visuals, is_selected: bool) -> DancerListItemColors {
    let (background, border, title) = if is_selected {
        (
            visuals.selection.bg_fill,
            visuals.selection.stroke.color,
            visuals.strong_text_color(),
        )
    } else {
        (
            visuals.extreme_bg_color,
            visuals.widgets.noninteractive.bg_stroke.color,
            visuals.text_color(),
        )
    };

    DancerListItemColors {
        background,
        border,
        title,
        subtitle: visuals.weak_text_color(),
    }
}

#[must_use]
pub fn row_size(available_width: f32) -> Vec2 {
    vec2(available_width, ROW_HEIGHT_PX)
}
