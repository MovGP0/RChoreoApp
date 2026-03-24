pub use super::geometry::pane_list_height;
pub use super::tokens::pane_corner_radius_token;
pub use super::tokens::pane_inner_padding_token;
pub use super::tokens::pane_spacing_token;
pub use super::widget::draw;

use crate::dancers::state::DancerState;

#[derive(Debug, Clone, PartialEq)]
pub struct DancersPaneViewUiState<'a> {
    pub dancer_items: &'a [DancerState],
    pub selected_dancer_index: Option<usize>,
    pub can_delete_dancer: bool,
    pub title_text: &'a str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DancersPaneViewAction {
    SelectDancer { index: usize },
    AddDancer,
    DeleteDancer,
}
