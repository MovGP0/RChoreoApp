use choreo_master_mobile_json::Color;

use crate::behavior::{Behavior, CompositeDisposable};
use crate::logging::BehaviorLog;
use crate::preferences::Preferences;

use super::view_model::{
    default_primary_color, default_secondary_color, default_tertiary_color, SettingsViewModel,
    ThemeMode,
};

pub trait MaterialSchemeUpdater {
    fn update(&self, settings: &SettingsViewModel, preferences: &dyn Preferences);
}

pub struct LoadSettingsPreferencesBehavior<P: Preferences> {
    preferences: P,
}

impl<P: Preferences> LoadSettingsPreferencesBehavior<P> {
    pub fn new(preferences: P) -> Self {
        Self { preferences }
    }

    pub fn load(&self, view_model: &mut SettingsViewModel) {
        let stored_theme = self
            .preferences
            .get_string(choreo_models::SettingsPreferenceKeys::THEME, "Light");
        view_model.theme_mode = if stored_theme == "Dark" {
            ThemeMode::Dark
        } else {
            ThemeMode::Light
        };

        view_model.use_system_theme = self
            .preferences
            .get_bool(choreo_models::SettingsPreferenceKeys::USE_SYSTEM_THEME, true);
        view_model.use_primary_color = self
            .preferences
            .get_bool(choreo_models::SettingsPreferenceKeys::USE_PRIMARY_COLOR, false);
        view_model.use_secondary_color = self
            .preferences
            .get_bool(choreo_models::SettingsPreferenceKeys::USE_SECONDARY_COLOR, false)
            && view_model.use_primary_color;
        view_model.use_tertiary_color = self
            .preferences
            .get_bool(choreo_models::SettingsPreferenceKeys::USE_TERTIARY_COLOR, false)
            && view_model.use_secondary_color;

        view_model.primary_color = self.get_color_from_preferences(
            choreo_models::SettingsPreferenceKeys::PRIMARY_COLOR,
            default_primary_color(),
        );
        view_model.secondary_color = self.get_color_from_preferences(
            choreo_models::SettingsPreferenceKeys::SECONDARY_COLOR,
            default_secondary_color(),
        );
        view_model.tertiary_color = self.get_color_from_preferences(
            choreo_models::SettingsPreferenceKeys::TERTIARY_COLOR,
            default_tertiary_color(),
        );
    }

    fn get_color_from_preferences(&self, key: &str, fallback: Color) -> Color {
        let stored = self.preferences.get_string(key, "");
        if !stored.trim().is_empty()
            && let Some(parsed) = Color::from_hex(&stored)
        {
            return parsed;
        }

        fallback
    }
}

impl<P: Preferences> Behavior<SettingsViewModel> for LoadSettingsPreferencesBehavior<P> {
    fn activate(&self, view_model: &mut SettingsViewModel, _disposables: &mut CompositeDisposable) {
        BehaviorLog::behavior_activated(
            "LoadSettingsPreferencesBehavior",
            "SettingsViewModel",
        );
        self.load(view_model);
    }
}

pub struct ColorPreferencesBehavior<P: Preferences, U: MaterialSchemeUpdater> {
    preferences: P,
    updater: U,
}

impl<P: Preferences, U: MaterialSchemeUpdater> ColorPreferencesBehavior<P, U> {
    pub fn new(preferences: P, updater: U) -> Self {
        Self { preferences, updater }
    }

    pub fn update_use_primary_color(&self, view_model: &mut SettingsViewModel, enabled: bool) {
        view_model.use_primary_color = enabled;
        self.preferences.set_bool(
            choreo_models::SettingsPreferenceKeys::USE_PRIMARY_COLOR,
            enabled,
        );

        if !enabled {
            self.preferences
                .remove(choreo_models::SettingsPreferenceKeys::PRIMARY_COLOR);
            view_model.use_secondary_color = false;
            view_model.use_tertiary_color = false;
        }

        self.updater.update(view_model, &self.preferences);
    }

    pub fn update_use_secondary_color(&self, view_model: &mut SettingsViewModel, enabled: bool) {
        if enabled && !view_model.use_primary_color {
            view_model.use_secondary_color = false;
            return;
        }

        view_model.use_secondary_color = enabled;
        self.preferences.set_bool(
            choreo_models::SettingsPreferenceKeys::USE_SECONDARY_COLOR,
            enabled,
        );

        if !enabled {
            self.preferences
                .remove(choreo_models::SettingsPreferenceKeys::SECONDARY_COLOR);
            view_model.use_tertiary_color = false;
        }

        self.updater.update(view_model, &self.preferences);
    }

    pub fn update_use_tertiary_color(&self, view_model: &mut SettingsViewModel, enabled: bool) {
        if enabled && !view_model.use_secondary_color {
            view_model.use_tertiary_color = false;
            return;
        }

        view_model.use_tertiary_color = enabled;
        self.preferences.set_bool(
            choreo_models::SettingsPreferenceKeys::USE_TERTIARY_COLOR,
            enabled,
        );

        if !enabled {
            self.preferences
                .remove(choreo_models::SettingsPreferenceKeys::TERTIARY_COLOR);
        }

        self.updater.update(view_model, &self.preferences);
    }

    pub fn update_primary_color(&self, view_model: &mut SettingsViewModel, color: Color) {
        let hex = color.to_hex();
        view_model.primary_color = color;
        if view_model.use_primary_color {
            self.preferences.set_string(
                choreo_models::SettingsPreferenceKeys::PRIMARY_COLOR,
                hex,
            );
            self.updater.update(view_model, &self.preferences);
        }
    }

    pub fn update_secondary_color(&self, view_model: &mut SettingsViewModel, color: Color) {
        let hex = color.to_hex();
        view_model.secondary_color = color;
        if view_model.use_secondary_color {
            self.preferences.set_string(
                choreo_models::SettingsPreferenceKeys::SECONDARY_COLOR,
                hex,
            );
            self.updater.update(view_model, &self.preferences);
        }
    }

    pub fn update_tertiary_color(&self, view_model: &mut SettingsViewModel, color: Color) {
        let hex = color.to_hex();
        view_model.tertiary_color = color;
        if view_model.use_tertiary_color {
            self.preferences.set_string(
                choreo_models::SettingsPreferenceKeys::TERTIARY_COLOR,
                hex,
            );
            self.updater.update(view_model, &self.preferences);
        }
    }
}

impl<P: Preferences, U: MaterialSchemeUpdater> Behavior<SettingsViewModel>
    for ColorPreferencesBehavior<P, U>
{
    fn activate(&self, _view_model: &mut SettingsViewModel, _disposables: &mut CompositeDisposable) {
        BehaviorLog::behavior_activated("ColorPreferencesBehavior", "SettingsViewModel");
    }
}

pub struct SwitchDarkLightModeBehavior<P: Preferences, U: MaterialSchemeUpdater> {
    preferences: P,
    updater: U,
}

impl<P: Preferences, U: MaterialSchemeUpdater> SwitchDarkLightModeBehavior<P, U> {
    pub fn new(preferences: P, updater: U) -> Self {
        Self { preferences, updater }
    }

    pub fn switch_theme_mode(&self, view_model: &mut SettingsViewModel, is_dark: bool) {
        view_model.theme_mode = if is_dark {
            ThemeMode::Dark
        } else {
            ThemeMode::Light
        };

        let theme = if is_dark { "Dark" } else { "Light" };
        self.preferences
            .set_string(choreo_models::SettingsPreferenceKeys::THEME, theme.to_string());
        self.updater.update(view_model, &self.preferences);
    }
}

impl<P: Preferences, U: MaterialSchemeUpdater> Behavior<SettingsViewModel>
    for SwitchDarkLightModeBehavior<P, U>
{
    fn activate(&self, _view_model: &mut SettingsViewModel, _disposables: &mut CompositeDisposable) {
        BehaviorLog::behavior_activated(
            "SwitchDarkLightModeBehavior",
            "SettingsViewModel",
        );
    }
}

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
        Box::new(LoadSettingsPreferencesBehavior::new(preferences.clone())),
        Box::new(SwitchDarkLightModeBehavior::new(
            preferences.clone(),
            updater.clone(),
        )),
        Box::new(ColorPreferencesBehavior::new(preferences, updater)),
    ]
}
