use egui::Rect;
use egui::Ui;
use egui::pos2;
use egui::vec2;

use crate::choreography_settings;
use crate::choreo_main::state::ChoreoMainState;
use crate::material::components::drawer_host::state::DrawerHostOpenMode;
use crate::material::components::drawer_host::state::DrawerHostState;

pub(super) const TOP_BAR_HEIGHT_PX: f32 = 84.0;
pub(super) const DRAWER_WIDTH_LEFT_PX: f32 = 324.0;
pub(super) const AUDIO_PANEL_HEIGHT_PX: f32 = 84.0;
pub(super) const GRID_12_PX: f32 = 12.0;

#[must_use]
pub fn drawer_host_state(_viewport_size: egui::Vec2, state: &ChoreoMainState) -> DrawerHostState {
    DrawerHostState {
        left_drawer_width: DRAWER_WIDTH_LEFT_PX,
        right_drawer_width: choreography_settings::ui::drawer_width_token(),
        responsive_breakpoint: 900.0,
        open_mode: DrawerHostOpenMode::Standard,
        top_inset: 0.0,
        left_close_on_click_away: false,
        right_close_on_click_away: false,
        inline_left: false,
        is_left_open: state.is_nav_open,
        is_right_open: state.is_choreography_settings_open,
        ..DrawerHostState::default()
    }
}

#[must_use]
pub fn top_bar_rect(page_rect: Rect) -> Rect {
    Rect::from_min_size(page_rect.min, vec2(page_rect.width(), TOP_BAR_HEIGHT_PX))
}

#[must_use]
pub fn drawer_host_rect(page_rect: Rect, audio_panel_height: f32) -> Rect {
    let host_top = (page_rect.min.y + TOP_BAR_HEIGHT_PX).min(page_rect.max.y);
    let host_bottom = (page_rect.max.y - audio_panel_height).max(host_top);
    Rect::from_min_max(
        pos2(page_rect.min.x, host_top),
        pos2(page_rect.max.x, host_bottom),
    )
}

#[must_use]
pub fn shell_rect(ui: &Ui) -> Rect {
    ui.max_rect()
}

#[must_use]
pub fn audio_panel_height_px(is_audio_player_open: bool) -> f32 {
    if is_audio_player_open {
        AUDIO_PANEL_HEIGHT_PX
    } else {
        0.0
    }
}

#[must_use]
pub fn audio_panel_rect(page_rect: Rect, audio_panel_height: f32) -> Rect {
    let panel_top = (page_rect.max.y - audio_panel_height).max(page_rect.min.y);
    Rect::from_min_max(pos2(page_rect.min.x, panel_top), page_rect.max)
}
