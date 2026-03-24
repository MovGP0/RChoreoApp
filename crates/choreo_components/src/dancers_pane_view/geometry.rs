use egui::vec2;

use crate::dancers_pane_view::tokens::pane_spacing_token;

#[must_use]
pub fn pane_list_height(available_height: f32) -> f32 {
    (available_height - pane_spacing_token() - 48.0).max(0.0)
}

#[must_use]
pub(super) fn pane_list_size(available_width: f32, available_height: f32) -> egui::Vec2 {
    vec2(available_width, pane_list_height(available_height))
}
