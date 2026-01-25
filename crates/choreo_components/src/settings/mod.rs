mod behaviors;
mod material;
mod view_model;

pub use behaviors::{
    build_settings_behaviors, build_settings_view_model, ColorPreferencesBehavior,
    LoadSettingsPreferencesBehavior, SettingsDependencies, SwitchDarkLightModeBehavior,
};
pub use material::{
    MaterialScheme, MaterialSchemeApplier, MaterialSchemeHelper, MaterialSchemes,
};
pub use view_model::{
    default_primary_color, default_secondary_color, default_tertiary_color,
    BooleanNegationConverter, SettingsViewModel, ThemeMode,
};

pub use behaviors::MaterialSchemeUpdater;
