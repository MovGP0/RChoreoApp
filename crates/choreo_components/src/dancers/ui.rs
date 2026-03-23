use crate::dancer_settings_page;
use crate::dancers::actions::DancersAction;
use crate::dancers::state::DancersState;
use crate::dancers_pane_view::ui::DancersPaneViewAction;

pub use crate::dancer_settings_page::action::SwapDialogAction;
pub use crate::dancer_settings_page::state::SwapDialogViewModel;
pub use crate::dancer_settings_page::state::build_swap_dialog_view_model;
pub use crate::dancer_settings_page::state::dancer_option_labels;
pub use crate::dancer_settings_page::state::dancer_role_details_text;
pub use crate::dancer_settings_page::state::dancer_supporting_text;
pub use crate::dancer_settings_page::state::icon_option_labels;
pub use crate::dancer_settings_page::state::role_option_labels;
pub use crate::dancer_settings_page::state::selected_dancer_color_picker_state;
pub use crate::dancer_settings_page::state::selected_dancer_index;
pub use crate::dancer_settings_page::state::selected_icon_index;
pub use crate::dancer_settings_page::state::selected_role_index;
pub use crate::dancer_settings_page::ui::card_corner_radius_token;
pub use crate::dancer_settings_page::ui::content_max_width_token;
pub use crate::dancer_settings_page::ui::content_outer_margin_token;
pub use crate::dancer_settings_page::ui::content_spacing_token;
pub use crate::dancer_settings_page::ui::content_top_inset_token;
pub use crate::dancer_settings_page::ui::draw_swap_dialog_panel;
pub use crate::dancer_settings_page::ui::drawer_host_state;
pub use crate::dancer_settings_page::ui::dropdown_height_token;
pub use crate::dancer_settings_page::ui::footer_content_padding_token;
pub use crate::dancer_settings_page::ui::footer_height_token;
pub use crate::dancer_settings_page::ui::footer_inner_height_token;
pub use crate::dancer_settings_page::ui::footer_rect;
pub use crate::dancer_settings_page::ui::main_content_rect;
pub use crate::dancer_settings_page::ui::scroll_rect;
pub use crate::dancer_settings_page::ui::shell_rect;
pub use crate::dancer_settings_page::ui::top_bar_height_token;
pub use crate::dancer_settings_page::ui::top_bar_rect;
pub use crate::dancer_settings_page::ui::top_bar_title_role;
pub use crate::dancer_settings_page::ui::uses_scrollable_content_shell;

#[must_use]
pub fn draw(ui: &mut egui::Ui, state: &DancersState) -> Vec<DancersAction> {
    dancer_settings_page::ui::draw(ui, state)
        .into_iter()
        .flat_map(dancer_settings_page::reducer::map_action)
        .collect()
}

#[must_use]
pub fn map_swap_dialog_action(action: SwapDialogAction) -> DancersAction {
    dancer_settings_page::reducer::map_action(
        dancer_settings_page::reducer::map_swap_dialog_action(action),
    )
    .into_iter()
    .next()
    .expect("swap dialog action should map to exactly one dancers action")
}

#[must_use]
pub fn map_pane_action(action: DancersPaneViewAction) -> DancersAction {
    dancer_settings_page::reducer::map_action(dancer_settings_page::ui::map_pane_action(action))
        .into_iter()
        .next()
        .expect("pane action should map to exactly one dancers action")
}
