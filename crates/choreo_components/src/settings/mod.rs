mod color_preferences_behavior;
mod load_settings_preferences_behavior;
mod material;
mod settings_view_model;
mod switch_dark_light_mode_behavior;
mod types;

use crate::behavior::Behavior;
use crate::preferences::Preferences;

pub use color_preferences_behavior::ColorPreferencesBehavior;
pub use load_settings_preferences_behavior::LoadSettingsPreferencesBehavior;
pub use material::{
    MaterialScheme, MaterialSchemeApplier, MaterialSchemeHelper, MaterialSchemes,
};
pub use settings_view_model::{
    default_primary_color, default_secondary_color, default_tertiary_color,
    BooleanNegationConverter, SettingsViewModel, ThemeMode,
};
pub use switch_dark_light_mode_behavior::SwitchDarkLightModeBehavior;
pub use types::MaterialSchemeUpdater;

pub struct SettingsDependencies<P: Preferences, U: MaterialSchemeUpdater> {
    pub preferences: P,
    pub scheme_updater: U,
}

pub fn build_settings_view_model<
    P: Preferences + Clone + 'static,
    U: MaterialSchemeUpdater + Clone + 'static,
>(
    deps: SettingsDependencies<P, U>,
) -> SettingsViewModel {
    SettingsViewModel::new(build_settings_behaviors(deps))
}

pub fn build_settings_behaviors<
    P: Preferences + Clone + 'static,
    U: MaterialSchemeUpdater + Clone + 'static,
>(
    deps: SettingsDependencies<P, U>,
) -> Vec<Box<dyn Behavior<SettingsViewModel>>> {
    let preferences = deps.preferences;
    let updater = deps.scheme_updater;

    vec![
        Box::new(LoadSettingsPreferencesBehavior::new(
            preferences.clone(),
            updater.clone(),
        )),
        Box::new(SwitchDarkLightModeBehavior::new(
            preferences.clone(),
            updater.clone(),
        )),
        Box::new(ColorPreferencesBehavior::new(preferences, updater)),
    ]
}
