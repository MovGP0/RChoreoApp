use crate::preferences::Preferences;

use super::settings_view_model::SettingsViewModel;

pub trait MaterialSchemeUpdater {
    fn update(&self, settings: &SettingsViewModel, preferences: &dyn Preferences);
}
