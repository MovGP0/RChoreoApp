use crate::i18n::t;

use super::state::InteractionMode;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NavBarTranslations {
    pub toggle_navigation_tooltip: String,
    pub open_settings_tooltip: String,
    pub reset_floor_viewport_tooltip: String,
    pub open_image_tooltip: String,
    pub open_audio_tooltip: String,
    pub mode_label: String,
    pub mode_view: String,
    pub mode_move: String,
    pub mode_rotate_around_center: String,
    pub mode_rotate_around_dancer: String,
    pub mode_scale: String,
    pub mode_line_of_sight: String,
}

#[must_use]
pub fn nav_bar_translations(locale: &str) -> NavBarTranslations {
    NavBarTranslations {
        toggle_navigation_tooltip: t(locale, "MainToggleNavTooltip"),
        open_settings_tooltip: t(locale, "MainOpenSettingsTooltip"),
        reset_floor_viewport_tooltip: t(locale, "MainHomeTooltip"),
        open_image_tooltip: t(locale, "MainOpenImageTooltip"),
        open_audio_tooltip: t(locale, "MainOpenAudioTooltip"),
        mode_label: t(locale, "ModeLabel"),
        mode_view: t(locale, "ModeView"),
        mode_move: t(locale, "ModeMove"),
        mode_rotate_around_center: t(locale, "ModeRotateAroundCenter"),
        mode_rotate_around_dancer: t(locale, "ModeRotateAroundDancer"),
        mode_scale: t(locale, "ModeScale"),
        mode_line_of_sight: t(locale, "ModeLineOfSight"),
    }
}

#[must_use]
pub fn mode_text(strings: &NavBarTranslations, mode: InteractionMode) -> &str {
    match mode {
        InteractionMode::View => strings.mode_view.as_str(),
        InteractionMode::Move => strings.mode_move.as_str(),
        InteractionMode::RotateAroundCenter => strings.mode_rotate_around_center.as_str(),
        InteractionMode::RotateAroundDancer => strings.mode_rotate_around_dancer.as_str(),
        InteractionMode::Scale => strings.mode_scale.as_str(),
        InteractionMode::LineOfSight => strings.mode_line_of_sight.as_str(),
    }
}
