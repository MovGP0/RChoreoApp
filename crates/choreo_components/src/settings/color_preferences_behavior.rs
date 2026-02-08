use choreo_master_mobile_json::Color;
use std::time::Duration;

use crossbeam_channel::Receiver;
use slint::TimerMode;

use crate::behavior::{Behavior, CompositeDisposable, TimerDisposable};
use crate::logging::BehaviorLog;
use crate::preferences::Preferences;
use nject::injectable;

use super::messages::{
    UpdatePrimaryColorHexCommand, UpdateSecondaryColorHexCommand, UpdateTertiaryColorHexCommand,
    UpdateUsePrimaryColorCommand, UpdateUseSecondaryColorCommand, UpdateUseTertiaryColorCommand,
};
use super::settings_view_model::SettingsViewModel;
use super::types::MaterialSchemeUpdater;

pub(super) struct ColorPreferencesReceivers {
    update_use_primary_color_receiver: Receiver<UpdateUsePrimaryColorCommand>,
    update_use_secondary_color_receiver: Receiver<UpdateUseSecondaryColorCommand>,
    update_use_tertiary_color_receiver: Receiver<UpdateUseTertiaryColorCommand>,
    update_primary_color_hex_receiver: Receiver<UpdatePrimaryColorHexCommand>,
    update_secondary_color_hex_receiver: Receiver<UpdateSecondaryColorHexCommand>,
    update_tertiary_color_hex_receiver: Receiver<UpdateTertiaryColorHexCommand>,
}

impl ColorPreferencesReceivers {
    pub(super) fn new(
        update_use_primary_color_receiver: Receiver<UpdateUsePrimaryColorCommand>,
        update_use_secondary_color_receiver: Receiver<UpdateUseSecondaryColorCommand>,
        update_use_tertiary_color_receiver: Receiver<UpdateUseTertiaryColorCommand>,
        update_primary_color_hex_receiver: Receiver<UpdatePrimaryColorHexCommand>,
        update_secondary_color_hex_receiver: Receiver<UpdateSecondaryColorHexCommand>,
        update_tertiary_color_hex_receiver: Receiver<UpdateTertiaryColorHexCommand>,
    ) -> Self {
        Self {
            update_use_primary_color_receiver,
            update_use_secondary_color_receiver,
            update_use_tertiary_color_receiver,
            update_primary_color_hex_receiver,
            update_secondary_color_hex_receiver,
            update_tertiary_color_hex_receiver,
        }
    }
}

#[injectable]
#[inject(
    |preferences: P,
     updater: U,
     receivers: ColorPreferencesReceivers| {
        Self::new(preferences, updater, receivers)
    }
)]
pub struct ColorPreferencesBehavior<
    P: Preferences + Clone + 'static,
    U: MaterialSchemeUpdater + Clone + 'static,
> {
    preferences: P,
    updater: U,
    update_use_primary_color_receiver: Receiver<UpdateUsePrimaryColorCommand>,
    update_use_secondary_color_receiver: Receiver<UpdateUseSecondaryColorCommand>,
    update_use_tertiary_color_receiver: Receiver<UpdateUseTertiaryColorCommand>,
    update_primary_color_hex_receiver: Receiver<UpdatePrimaryColorHexCommand>,
    update_secondary_color_hex_receiver: Receiver<UpdateSecondaryColorHexCommand>,
    update_tertiary_color_hex_receiver: Receiver<UpdateTertiaryColorHexCommand>,
}

impl<P: Preferences + Clone + 'static, U: MaterialSchemeUpdater + Clone + 'static>
    ColorPreferencesBehavior<P, U>
{
    pub(super) fn new(preferences: P, updater: U, receivers: ColorPreferencesReceivers) -> Self {
        Self {
            preferences,
            updater,
            update_use_primary_color_receiver: receivers.update_use_primary_color_receiver,
            update_use_secondary_color_receiver: receivers.update_use_secondary_color_receiver,
            update_use_tertiary_color_receiver: receivers.update_use_tertiary_color_receiver,
            update_primary_color_hex_receiver: receivers.update_primary_color_hex_receiver,
            update_secondary_color_hex_receiver: receivers.update_secondary_color_hex_receiver,
            update_tertiary_color_hex_receiver: receivers.update_tertiary_color_hex_receiver,
        }
    }

    fn update_use_primary_color(
        view_model: &mut SettingsViewModel,
        preferences: &P,
        updater: &U,
        enabled: bool,
    ) {
        view_model.use_primary_color = enabled;
        preferences.set_bool(
            choreo_models::SettingsPreferenceKeys::USE_PRIMARY_COLOR,
            enabled,
        );

        if !enabled {
            preferences.remove(choreo_models::SettingsPreferenceKeys::PRIMARY_COLOR);
            view_model.use_secondary_color = false;
            view_model.use_tertiary_color = false;
        }

        updater.update(view_model, preferences);
    }

    fn update_use_secondary_color(
        view_model: &mut SettingsViewModel,
        preferences: &P,
        updater: &U,
        enabled: bool,
    ) {
        if enabled && !view_model.use_primary_color {
            view_model.use_secondary_color = false;
            return;
        }

        view_model.use_secondary_color = enabled;
        preferences.set_bool(
            choreo_models::SettingsPreferenceKeys::USE_SECONDARY_COLOR,
            enabled,
        );

        if !enabled {
            preferences.remove(choreo_models::SettingsPreferenceKeys::SECONDARY_COLOR);
            view_model.use_tertiary_color = false;
        }

        updater.update(view_model, preferences);
    }

    fn update_use_tertiary_color(
        view_model: &mut SettingsViewModel,
        preferences: &P,
        updater: &U,
        enabled: bool,
    ) {
        if enabled && !view_model.use_secondary_color {
            view_model.use_tertiary_color = false;
            return;
        }

        view_model.use_tertiary_color = enabled;
        preferences.set_bool(
            choreo_models::SettingsPreferenceKeys::USE_TERTIARY_COLOR,
            enabled,
        );

        if !enabled {
            preferences.remove(choreo_models::SettingsPreferenceKeys::TERTIARY_COLOR);
        }

        updater.update(view_model, preferences);
    }

    fn update_primary_color_hex(
        view_model: &mut SettingsViewModel,
        preferences: &P,
        updater: &U,
        value: String,
    ) {
        let Some(color) = Color::from_hex(value.trim()) else {
            return;
        };
        let hex = color.to_hex();
        view_model.primary_color = color;
        if view_model.use_primary_color {
            preferences.set_string(choreo_models::SettingsPreferenceKeys::PRIMARY_COLOR, hex);
            updater.update(view_model, preferences);
        }
    }

    fn update_secondary_color_hex(
        view_model: &mut SettingsViewModel,
        preferences: &P,
        updater: &U,
        value: String,
    ) {
        let Some(color) = Color::from_hex(value.trim()) else {
            return;
        };
        let hex = color.to_hex();
        view_model.secondary_color = color;
        if view_model.use_secondary_color {
            preferences.set_string(choreo_models::SettingsPreferenceKeys::SECONDARY_COLOR, hex);
            updater.update(view_model, preferences);
        }
    }

    fn update_tertiary_color_hex(
        view_model: &mut SettingsViewModel,
        preferences: &P,
        updater: &U,
        value: String,
    ) {
        let Some(color) = Color::from_hex(value.trim()) else {
            return;
        };
        let hex = color.to_hex();
        view_model.tertiary_color = color;
        if view_model.use_tertiary_color {
            preferences.set_string(choreo_models::SettingsPreferenceKeys::TERTIARY_COLOR, hex);
            updater.update(view_model, preferences);
        }
    }
}

impl<P: Preferences + Clone + 'static, U: MaterialSchemeUpdater + Clone + 'static>
    Behavior<SettingsViewModel> for ColorPreferencesBehavior<P, U>
{
    fn activate(&self, view_model: &mut SettingsViewModel, disposables: &mut CompositeDisposable) {
        BehaviorLog::behavior_activated("ColorPreferencesBehavior", "SettingsViewModel");
        let Some(view_model_handle) = view_model.self_handle().and_then(|handle| handle.upgrade())
        else {
            return;
        };

        let update_use_primary_color_receiver = self.update_use_primary_color_receiver.clone();
        let update_use_secondary_color_receiver = self.update_use_secondary_color_receiver.clone();
        let update_use_tertiary_color_receiver = self.update_use_tertiary_color_receiver.clone();
        let update_primary_color_hex_receiver = self.update_primary_color_hex_receiver.clone();
        let update_secondary_color_hex_receiver = self.update_secondary_color_hex_receiver.clone();
        let update_tertiary_color_hex_receiver = self.update_tertiary_color_hex_receiver.clone();
        let preferences = self.preferences.clone();
        let updater = self.updater.clone();
        let timer = slint::Timer::default();
        timer.start(TimerMode::Repeated, Duration::from_millis(16), move || {
            while let Ok(command) = update_use_primary_color_receiver.try_recv() {
                let mut view_model = view_model_handle.borrow_mut();
                Self::update_use_primary_color(
                    &mut view_model,
                    &preferences,
                    &updater,
                    command.enabled,
                );
            }

            while let Ok(command) = update_use_secondary_color_receiver.try_recv() {
                let mut view_model = view_model_handle.borrow_mut();
                Self::update_use_secondary_color(
                    &mut view_model,
                    &preferences,
                    &updater,
                    command.enabled,
                );
            }

            while let Ok(command) = update_use_tertiary_color_receiver.try_recv() {
                let mut view_model = view_model_handle.borrow_mut();
                Self::update_use_tertiary_color(
                    &mut view_model,
                    &preferences,
                    &updater,
                    command.enabled,
                );
            }

            while let Ok(command) = update_primary_color_hex_receiver.try_recv() {
                let mut view_model = view_model_handle.borrow_mut();
                Self::update_primary_color_hex(
                    &mut view_model,
                    &preferences,
                    &updater,
                    command.value,
                );
            }

            while let Ok(command) = update_secondary_color_hex_receiver.try_recv() {
                let mut view_model = view_model_handle.borrow_mut();
                Self::update_secondary_color_hex(
                    &mut view_model,
                    &preferences,
                    &updater,
                    command.value,
                );
            }

            while let Ok(command) = update_tertiary_color_hex_receiver.try_recv() {
                let mut view_model = view_model_handle.borrow_mut();
                Self::update_tertiary_color_hex(
                    &mut view_model,
                    &preferences,
                    &updater,
                    command.value,
                );
            }
        });
        disposables.add(Box::new(TimerDisposable::new(timer)));
    }
}
