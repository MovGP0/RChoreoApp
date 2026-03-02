use egui::Color32;

#[derive(Debug, Clone, PartialEq)]
pub struct DrawerHostState {
    pub left_drawer_width: f32,
    pub right_drawer_width: f32,
    pub top_drawer_height: f32,
    pub bottom_drawer_height: f32,
    pub top_inset: f32,
    pub inline_left: bool,
    pub is_left_open: bool,
    pub is_right_open: bool,
    pub is_top_open: bool,
    pub is_bottom_open: bool,
    pub left_close_on_click_away: bool,
    pub right_close_on_click_away: bool,
    pub top_close_on_click_away: bool,
    pub bottom_close_on_click_away: bool,
    pub overlay_color: Color32,
    pub drawer_background: Color32,
}

impl Default for DrawerHostState {
    fn default() -> Self {
        Self {
            left_drawer_width: 320.0,
            right_drawer_width: 320.0,
            top_drawer_height: 320.0,
            bottom_drawer_height: 320.0,
            top_inset: 0.0,
            inline_left: false,
            is_left_open: false,
            is_right_open: false,
            is_top_open: false,
            is_bottom_open: false,
            left_close_on_click_away: true,
            right_close_on_click_away: true,
            top_close_on_click_away: true,
            bottom_close_on_click_away: true,
            overlay_color: Color32::from_black_alpha(160),
            drawer_background: Color32::from_gray(46),
        }
    }
}
