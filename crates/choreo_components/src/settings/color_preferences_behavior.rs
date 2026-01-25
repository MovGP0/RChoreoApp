use choreo_master_mobile_json::Color;

use crate::behavior::{Behavior, CompositeDisposable};
use crate::logging::BehaviorLog;
use crate::preferences::Preferences;
use nject::injectable;

use super::settings_view_model::SettingsViewModel;
use super::types::MaterialSchemeUpdater;

#[injectable]
#[inject(|preferences: P, updater: U| Self::new(preferences, updater))]
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
