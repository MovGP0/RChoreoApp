use egui::Rect;
use egui::Ui;
use egui::pos2;
use egui::vec2;

use super::tokens::content_max_width_token;
use super::tokens::content_outer_margin_token;
use super::tokens::content_top_inset_token;
use super::tokens::footer_height_token;
use super::tokens::top_bar_height_token;

#[must_use]
pub fn shell_rect(ui: &Ui) -> Rect {
    ui.max_rect()
}

#[must_use]
pub fn top_bar_rect(page_rect: Rect) -> Rect {
    Rect::from_min_size(
        page_rect.min,
        vec2(page_rect.width(), top_bar_height_token()),
    )
}

#[must_use]
pub fn main_content_rect(page_rect: Rect) -> Rect {
    Rect::from_min_max(
        pos2(page_rect.left(), top_bar_rect(page_rect).bottom()),
        page_rect.right_bottom(),
    )
}

#[must_use]
pub fn content_column_width(surface_rect: Rect) -> f32 {
    let max_content_width = (surface_rect.width() - (content_outer_margin_token() * 2.0)).max(0.0);
    content_max_width_token().min(max_content_width)
}

#[must_use]
pub fn content_column_left(surface_rect: Rect) -> f32 {
    surface_rect.left() + content_outer_margin_token()
}

#[must_use]
pub fn content_column_right(surface_rect: Rect) -> f32 {
    content_column_left(surface_rect) + content_column_width(surface_rect)
}

#[must_use]
pub fn footer_rect(surface_rect: Rect) -> Rect {
    Rect::from_min_max(
        pos2(
            surface_rect.left(),
            surface_rect.bottom() - footer_height_token(),
        ),
        surface_rect.right_bottom(),
    )
}

#[must_use]
pub fn scroll_rect(surface_rect: Rect) -> Rect {
    let footer_rect = footer_rect(surface_rect);
    Rect::from_min_max(
        pos2(
            content_column_left(surface_rect),
            surface_rect.top() + content_top_inset_token(),
        ),
        pos2(
            content_column_right(surface_rect),
            footer_rect.top() - content_outer_margin_token(),
        ),
    )
}
