use std::time::Duration;

use crossbeam_channel::Receiver;
use slint::TimerMode;

use crate::behavior::{Behavior, CompositeDisposable, TimerDisposable};
use crate::logging::BehaviorLog;
use crate::preferences::Preferences;
use nject::injectable;

use super::messages::{SwitchThemeModeCommand, UpdateUseSystemThemeCommand};
use super::settings_view_model::{SettingsViewModel, ThemeMode};
use super::types::MaterialSchemeUpdater;

#[injectable]
#[inject(
    |preferences: P,
     updater: U,
     update_use_system_theme_receiver: Receiver<UpdateUseSystemThemeCommand>,
     switch_theme_mode_receiver: Receiver<SwitchThemeModeCommand>| {
        Self::new(
            preferences,
            updater,
            update_use_system_theme_receiver,
            switch_theme_mode_receiver,
        )
    }
)]
pub struct SwitchDarkLightModeBehavior<P: Preferences + Clone + 'static, U: MaterialSchemeUpdater + Clone + 'static> {
    preferences: P,
    updater: U,
    update_use_system_theme_receiver: Receiver<UpdateUseSystemThemeCommand>,
    switch_theme_mode_receiver: Receiver<SwitchThemeModeCommand>,
}

impl<P: Preferences + Clone + 'static, U: MaterialSchemeUpdater + Clone + 'static> SwitchDarkLightModeBehavior<P, U> {
    pub fn new(
        preferences: P,
        updater: U,
        update_use_system_theme_receiver: Receiver<UpdateUseSystemThemeCommand>,
        switch_theme_mode_receiver: Receiver<SwitchThemeModeCommand>,
    ) -> Self {
        Self {
            preferences,
            updater,
            update_use_system_theme_receiver,
            switch_theme_mode_receiver,
        }
    }

    fn apply_use_system_theme(
        view_model: &mut SettingsViewModel,
        preferences: &P,
        updater: &U,
        enabled: bool,
    ) {
        view_model.use_system_theme = enabled;
        preferences.set_bool(
            choreo_models::SettingsPreferenceKeys::USE_SYSTEM_THEME,
            enabled,
        );
        updater.update(view_model, preferences);
    }

    fn apply_theme_mode(
        view_model: &mut SettingsViewModel,
        preferences: &P,
        updater: &U,
        is_dark: bool,
    ) {
        view_model.theme_mode = if is_dark {
            ThemeMode::Dark
        } else {
            ThemeMode::Light
        };

        let theme = if is_dark { "Dark" } else { "Light" };
        preferences
            .set_string(choreo_models::SettingsPreferenceKeys::THEME, theme.to_string());
        updater.update(view_model, preferences);
    }
}

impl<P: Preferences + Clone + 'static, U: MaterialSchemeUpdater + Clone + 'static> Behavior<SettingsViewModel>
    for SwitchDarkLightModeBehavior<P, U>
{
    fn activate(&self, view_model: &mut SettingsViewModel, disposables: &mut CompositeDisposable) {
        BehaviorLog::behavior_activated(
            "SwitchDarkLightModeBehavior",
            "SettingsViewModel",
        );
        let Some(view_model_handle) = view_model.self_handle().and_then(|handle| handle.upgrade())
        else {
            return;
        };

        let update_use_system_theme_receiver = self.update_use_system_theme_receiver.clone();
        let switch_theme_mode_receiver = self.switch_theme_mode_receiver.clone();
        let preferences = self.preferences.clone();
        let updater = self.updater.clone();

        let timer = slint::Timer::default();
        timer.start(TimerMode::Repeated, Duration::from_millis(16), move || {
            while let Ok(command) = update_use_system_theme_receiver.try_recv() {
                let mut view_model = view_model_handle.borrow_mut();
                Self::apply_use_system_theme(
                    &mut view_model,
                    &preferences,
                    &updater,
                    command.enabled,
                );
            }

            while let Ok(command) = switch_theme_mode_receiver.try_recv() {
                let mut view_model = view_model_handle.borrow_mut();
                Self::apply_theme_mode(
                    &mut view_model,
                    &preferences,
                    &updater,
                    command.is_dark,
                );
            }
        });
        disposables.add(Box::new(TimerDisposable::new(timer)));
    }
}
