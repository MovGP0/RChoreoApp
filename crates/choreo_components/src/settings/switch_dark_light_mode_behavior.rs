use crate::behavior::{Behavior, CompositeDisposable};
use crate::logging::BehaviorLog;
use crate::preferences::Preferences;
use nject::injectable;

use super::settings_view_model::{SettingsViewModel, ThemeMode};
use super::types::MaterialSchemeUpdater;

#[injectable]
#[inject(|preferences: P, updater: U| Self::new(preferences, updater))]
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
