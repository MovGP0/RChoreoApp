use choreo_master_mobile_json::Color;

use crate::behavior::{Behavior, CompositeDisposable};
use crate::logging::BehaviorLog;
use crate::preferences::Preferences;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeMode {
    Light,
    Dark,
}

#[derive(Debug)]
pub struct SettingsViewModel {
    pub theme_mode: ThemeMode,
    pub use_system_theme: bool,
    pub use_primary_color: bool,
    pub use_secondary_color: bool,
    pub use_tertiary_color: bool,
    pub primary_color: Color,
    pub secondary_color: Color,
    pub tertiary_color: Color,
    disposables: CompositeDisposable,
}

impl SettingsViewModel {
    pub fn new(mut behaviors: Vec<Box<dyn Behavior<SettingsViewModel>>>) -> Self {
        let mut view_model = Self {
            theme_mode: ThemeMode::Light,
            use_system_theme: false,
            use_primary_color: false,
            use_secondary_color: false,
            use_tertiary_color: false,
            primary_color: default_primary_color(),
            secondary_color: default_secondary_color(),
            tertiary_color: default_tertiary_color(),
            disposables: CompositeDisposable::new(),
        };

        let mut disposables = CompositeDisposable::new();
        for behavior in behaviors.drain(..) {
            behavior.activate(&mut view_model, &mut disposables);
        }

        view_model.disposables = disposables;
        view_model
    }

    pub fn dispose(&mut self) {
        self.disposables.dispose_all();
    }
}

impl Default for SettingsViewModel {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}

pub fn default_primary_color() -> Color {
    Color {
        a: 255,
        r: 0x19,
        g: 0x76,
        b: 0xD2,
    }
}

pub fn default_secondary_color() -> Color {
    Color {
        a: 255,
        r: 0x67,
        g: 0x5A,
        b: 0x84,
    }
}

pub fn default_tertiary_color() -> Color {
    Color {
        a: 255,
        r: 0x82,
        g: 0x5A,
        b: 0x2C,
    }
}

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

    pub fn update_is_dark_mode(&self, view_model: &mut SettingsViewModel, is_dark: bool) {
        if view_model.use_system_theme {
            return;
        }

        view_model.theme_mode = if is_dark { ThemeMode::Dark } else { ThemeMode::Light };
        let theme = if is_dark { "Dark" } else { "Light" };
        self.preferences
            .set_string(choreo_models::SettingsPreferenceKeys::THEME, theme.to_string());
        self.updater.update(view_model, &self.preferences);
    }

    pub fn update_use_system_theme(&self, view_model: &mut SettingsViewModel, use_system: bool) {
        view_model.use_system_theme = use_system;
        self.preferences.set_bool(
            choreo_models::SettingsPreferenceKeys::USE_SYSTEM_THEME,
            use_system,
        );

        if use_system {
            self.updater.update(view_model, &self.preferences);
            return;
        }

        let is_dark = matches!(view_model.theme_mode, ThemeMode::Dark);
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
