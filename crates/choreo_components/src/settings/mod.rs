mod audio_backend_preferences_behavior;
mod color_preferences_behavior;
mod load_settings_preferences_behavior;
mod material;
mod messages;
mod settings_provider;
mod settings_view_model;
mod switch_dark_light_mode_behavior;
mod system_theme;
mod types;

use crate::preferences::Preferences;

pub use audio_backend_preferences_behavior::AudioBackendPreferencesBehavior;
pub use color_preferences_behavior::ColorPreferencesBehavior;
pub use load_settings_preferences_behavior::LoadSettingsPreferencesBehavior;
pub use material::{MaterialScheme, MaterialSchemeApplier, MaterialSchemeHelper, MaterialSchemes};
pub use settings_provider::SettingsProvider;
pub use settings_view_model::{
    BooleanNegationConverter, SettingsViewModel, SettingsViewModelActions, ThemeMode,
    default_primary_color, default_secondary_color, default_tertiary_color,
};
pub use switch_dark_light_mode_behavior::SwitchDarkLightModeBehavior;
pub use system_theme::{detect_system_theme_mode, supports_system_theme_toggle};
pub use types::MaterialSchemeUpdater;

pub struct SettingsDependencies<P: Preferences, U: MaterialSchemeUpdater> {
    pub preferences: P,
    pub scheme_updater: U,
}
