use choreo_master_mobile_json::Color;

use std::time::Duration;

use crossbeam_channel::Receiver;
use slint::TimerMode;

use crate::behavior::{Behavior, CompositeDisposable, TimerDisposable};
use crate::logging::BehaviorLog;
use crate::preferences::Preferences;
use nject::injectable;

use super::messages::ReloadSettingsCommand;
use super::settings_view_model::{
    default_primary_color, default_secondary_color, default_tertiary_color, SettingsViewModel,
    ThemeMode,
};
use super::types::MaterialSchemeUpdater;

#[injectable]
#[inject(
    |preferences: P, updater: U, reload_receiver: Receiver<ReloadSettingsCommand>| {
        Self::new(preferences, updater, reload_receiver)
    }
)]
pub struct LoadSettingsPreferencesBehavior<P: Preferences + Clone + 'static, U: MaterialSchemeUpdater + Clone + 'static> {
    preferences: P,
    updater: U,
    reload_receiver: Receiver<ReloadSettingsCommand>,
}

impl<P: Preferences + Clone + 'static, U: MaterialSchemeUpdater + Clone + 'static> LoadSettingsPreferencesBehavior<P, U> {
    pub fn new(
        preferences: P,
        updater: U,
        reload_receiver: Receiver<ReloadSettingsCommand>,
    ) -> Self {
        Self {
            preferences,
            updater,
            reload_receiver,
        }
    }

    fn load(&self, view_model: &mut SettingsViewModel) {
        Self::load_from(view_model, &self.preferences, &self.updater);
    }

    fn load_from(view_model: &mut SettingsViewModel, preferences: &P, updater: &U) {
        let stored_theme = preferences
            .get_string(choreo_models::SettingsPreferenceKeys::THEME, "Light");
        view_model.theme_mode = if stored_theme == "Dark" {
            ThemeMode::Dark
        } else {
            ThemeMode::Light
        };

        view_model.use_system_theme = preferences
            .get_bool(choreo_models::SettingsPreferenceKeys::USE_SYSTEM_THEME, true);
        view_model.use_primary_color = preferences
            .get_bool(choreo_models::SettingsPreferenceKeys::USE_PRIMARY_COLOR, false);
        view_model.use_secondary_color = preferences
            .get_bool(choreo_models::SettingsPreferenceKeys::USE_SECONDARY_COLOR, false)
            && view_model.use_primary_color;
        view_model.use_tertiary_color = preferences
            .get_bool(choreo_models::SettingsPreferenceKeys::USE_TERTIARY_COLOR, false)
            && view_model.use_secondary_color;

        view_model.primary_color = Self::get_color_from_preferences(
            preferences,
            choreo_models::SettingsPreferenceKeys::PRIMARY_COLOR,
            default_primary_color(),
        );
        view_model.secondary_color = Self::get_color_from_preferences(
            preferences,
            choreo_models::SettingsPreferenceKeys::SECONDARY_COLOR,
            default_secondary_color(),
        );
        view_model.tertiary_color = Self::get_color_from_preferences(
            preferences,
            choreo_models::SettingsPreferenceKeys::TERTIARY_COLOR,
            default_tertiary_color(),
        );

        updater.update(view_model, preferences);
    }

    fn get_color_from_preferences(preferences: &P, key: &str, fallback: Color) -> Color {
        let stored = preferences.get_string(key, "");
        if !stored.trim().is_empty()
            && let Some(parsed) = Color::from_hex(&stored)
        {
            return parsed;
        }

        fallback
    }
}

impl<P: Preferences + Clone + 'static, U: MaterialSchemeUpdater + Clone + 'static> Behavior<SettingsViewModel>
    for LoadSettingsPreferencesBehavior<P, U>
{
    fn activate(&self, view_model: &mut SettingsViewModel, disposables: &mut CompositeDisposable) {
        BehaviorLog::behavior_activated(
            "LoadSettingsPreferencesBehavior",
            "SettingsViewModel",
        );
        self.load(view_model);

        let Some(view_model_handle) = view_model.self_handle().and_then(|handle| handle.upgrade())
        else {
            return;
        };

        let reload_receiver = self.reload_receiver.clone();
        let preferences = self.preferences.clone();
        let updater = self.updater.clone();
        let timer = slint::Timer::default();
        timer.start(TimerMode::Repeated, Duration::from_millis(16), move || {
            while reload_receiver.try_recv().is_ok() {
                let mut view_model = view_model_handle.borrow_mut();
                Self::load_from(&mut view_model, &preferences, &updater);
            }
        });
        disposables.add(Box::new(TimerDisposable::new(timer)));
    }
}
