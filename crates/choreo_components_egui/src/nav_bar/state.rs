#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InteractionMode {
    View,
    Move,
    RotateAroundCenter,
    RotateAroundDancer,
    Scale,
    LineOfSight,
}

pub const MODE_OPTIONS: [InteractionMode; 6] = [
    InteractionMode::View,
    InteractionMode::Move,
    InteractionMode::RotateAroundCenter,
    InteractionMode::RotateAroundDancer,
    InteractionMode::Scale,
    InteractionMode::LineOfSight,
];

#[must_use]
pub fn mode_option_from_index(index: i32) -> Option<InteractionMode> {
    if index < 0 {
        return None;
    }

    MODE_OPTIONS.get(index as usize).copied()
}

#[must_use]
pub fn mode_index(mode: InteractionMode) -> i32 {
    MODE_OPTIONS
        .iter()
        .position(|candidate| *candidate == mode)
        .map(|index| index as i32)
        .unwrap_or(-1)
}

#[must_use]
pub fn all_modes() -> &'static [InteractionMode] {
    &MODE_OPTIONS
}

#[derive(Debug, Clone, PartialEq)]
pub struct NavBarState {
    pub selected_mode: InteractionMode,
    pub is_mode_selection_enabled: bool,
    pub nav_width: f32,
    pub is_nav_open: bool,
    pub is_audio_player_open: bool,
    pub is_choreography_settings_open: bool,
}

impl NavBarState {
    pub const DEFAULT_NAV_WIDTH: f32 = 280.0;
}

impl Default for NavBarState {
    fn default() -> Self {
        Self {
            selected_mode: InteractionMode::View,
            is_mode_selection_enabled: true,
            nav_width: 0.0,
            is_nav_open: false,
            is_audio_player_open: false,
            is_choreography_settings_open: false,
        }
    }
}
