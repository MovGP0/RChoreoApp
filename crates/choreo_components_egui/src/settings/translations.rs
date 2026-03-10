use crate::i18n::t;

#[derive(Debug, Clone)]
pub struct SettingsTranslations {
    pub title: String,
    pub navigate_back: String,
    pub theme: String,
    pub use_system_theme: String,
    pub dark_mode: String,
    pub light_mode: String,
    pub audio_backend: String,
    pub backend_rodio: String,
    pub backend_awedio: String,
    pub backend_browser: String,
    pub colors: String,
    pub primary_color: String,
    pub secondary_color: String,
    pub tertiary_color: String,
    pub argb_hint: String,
    pub value_slider: String,
    pub dock_left: String,
    pub dock_top: String,
    pub dock_right: String,
    pub dock_bottom: String,
}

#[must_use]
pub fn settings_translations(locale: &str) -> SettingsTranslations {
    SettingsTranslations {
        title: t(locale, "settings_title", "Settings"),
        navigate_back: t(locale, "settings_navigate_back", "Back"),
        theme: t(locale, "settings_theme_label", "Theme"),
        use_system_theme: t(
            locale,
            "settings_use_system_theme_label",
            "Use system theme",
        ),
        dark_mode: t(locale, "dark_mode_label", "Dark mode"),
        light_mode: t(locale, "light_mode_label", "Light mode"),
        audio_backend: t(locale, "settings_audio_backend_label", "Audio backend"),
        backend_rodio: t(locale, "settings_audio_backend_rodio_label", "Rodio"),
        backend_awedio: t(locale, "settings_audio_backend_awedio_label", "Awedio"),
        backend_browser: t(locale, "settings_audio_backend_browser_label", "Browser"),
        colors: t(locale, "settings_colors_label", "Colors"),
        primary_color: t(locale, "settings_primary_color_label", "Primary color"),
        secondary_color: t(locale, "settings_secondary_color_label", "Secondary color"),
        tertiary_color: t(locale, "settings_tertiary_color_label", "Tertiary color"),
        argb_hint: t(locale, "settings_color_argb_hint", "#AARRGGBB"),
        value_slider: t(locale, "color_picker_value_slider_label", "Value slider:"),
        dock_left: t(locale, "color_picker_dock_left", "Left"),
        dock_top: t(locale, "color_picker_dock_top", "Top"),
        dock_right: t(locale, "color_picker_dock_right", "Right"),
        dock_bottom: t(locale, "color_picker_dock_bottom", "Bottom"),
    }
}
