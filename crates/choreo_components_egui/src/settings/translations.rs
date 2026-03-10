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
        title: t(locale, "SettingsTitle"),
        navigate_back: t(locale, "SettingsNavigateBack"),
        theme: t(locale, "SettingsThemeLabel"),
        use_system_theme: t(locale, "SettingsUseSystemThemeLabel"),
        dark_mode: t(locale, "DarkModeLabel"),
        light_mode: t(locale, "LightModeLabel"),
        audio_backend: t(locale, "SettingsAudioBackendLabel"),
        backend_rodio: t(locale, "SettingsAudioBackendRodioLabel"),
        backend_awedio: t(locale, "SettingsAudioBackendAwedioLabel"),
        backend_browser: t(locale, "SettingsAudioBackendBrowserLabel"),
        colors: t(locale, "SettingsColorsLabel"),
        primary_color: t(locale, "SettingsPrimaryColorLabel"),
        secondary_color: t(locale, "SettingsSecondaryColorLabel"),
        tertiary_color: t(locale, "SettingsTertiaryColorLabel"),
        argb_hint: t(locale, "SettingsColorArgbHint"),
        value_slider: t(locale, "ColorPickerValueSliderLabel"),
        dock_left: t(locale, "ColorPickerDockLeft"),
        dock_top: t(locale, "ColorPickerDockTop"),
        dock_right: t(locale, "ColorPickerDockRight"),
        dock_bottom: t(locale, "ColorPickerDockBottom"),
    }
}
