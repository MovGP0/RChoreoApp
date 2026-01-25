use choreo_master_mobile_json::Color;

use crate::behavior::{Behavior, CompositeDisposable};
use crate::logging::BehaviorLog;
use crate::preferences::Preferences;

use super::settings_view_model::{
    default_primary_color, default_secondary_color, default_tertiary_color, SettingsViewModel,
    ThemeMode,
};

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
