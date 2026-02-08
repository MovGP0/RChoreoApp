use std::cell::RefCell;
use std::rc::Rc;

use crossbeam_channel::{Sender, unbounded};

use crate::behavior::Behavior;
use crate::preferences::Preferences;

use super::color_preferences_behavior::{ColorPreferencesBehavior, ColorPreferencesReceivers};
use super::load_settings_preferences_behavior::LoadSettingsPreferencesBehavior;
use super::messages::{
    ReloadSettingsCommand, SwitchThemeModeCommand, UpdatePrimaryColorHexCommand,
    UpdateSecondaryColorHexCommand, UpdateTertiaryColorHexCommand, UpdateUsePrimaryColorCommand,
    UpdateUseSecondaryColorCommand, UpdateUseSystemThemeCommand, UpdateUseTertiaryColorCommand,
};
use super::settings_view_model::{SettingsViewModel, SettingsViewModelActions};
use super::switch_dark_light_mode_behavior::SwitchDarkLightModeBehavior;
use super::{MaterialSchemeUpdater, SettingsDependencies};

pub struct SettingsProvider {
    settings_view_model: Rc<RefCell<SettingsViewModel>>,
    _reload_sender: Sender<ReloadSettingsCommand>,
}

impl SettingsProvider {
    pub fn new<P, U>(deps: SettingsDependencies<P, U>) -> Self
    where
        P: Preferences + Clone + 'static,
        U: MaterialSchemeUpdater + Clone + 'static,
    {
        let (reload_sender, reload_receiver) = unbounded();
        let (update_use_system_theme_sender, update_use_system_theme_receiver) = unbounded();
        let (switch_theme_mode_sender, switch_theme_mode_receiver) = unbounded();
        let (update_use_primary_color_sender, update_use_primary_color_receiver) = unbounded();
        let (update_use_secondary_color_sender, update_use_secondary_color_receiver) = unbounded();
        let (update_use_tertiary_color_sender, update_use_tertiary_color_receiver) = unbounded();
        let (update_primary_color_hex_sender, update_primary_color_hex_receiver) = unbounded();
        let (update_secondary_color_hex_sender, update_secondary_color_hex_receiver) = unbounded();
        let (update_tertiary_color_hex_sender, update_tertiary_color_hex_receiver) = unbounded();

        let preferences = deps.preferences;
        let updater = deps.scheme_updater;

        let behaviors: Vec<Box<dyn Behavior<SettingsViewModel>>> = vec![
            Box::new(LoadSettingsPreferencesBehavior::new(
                preferences.clone(),
                updater.clone(),
                reload_receiver,
            )),
            Box::new(SwitchDarkLightModeBehavior::new(
                preferences.clone(),
                updater.clone(),
                update_use_system_theme_receiver,
                switch_theme_mode_receiver,
            )),
            Box::new(ColorPreferencesBehavior::new(
                preferences,
                updater,
                ColorPreferencesReceivers::new(
                    update_use_primary_color_receiver,
                    update_use_secondary_color_receiver,
                    update_use_tertiary_color_receiver,
                    update_primary_color_hex_receiver,
                    update_secondary_color_hex_receiver,
                    update_tertiary_color_hex_receiver,
                ),
            )),
        ];

        let settings_view_model = Rc::new(RefCell::new(SettingsViewModel::new()));
        settings_view_model
            .borrow_mut()
            .set_self_handle(Rc::downgrade(&settings_view_model));
        SettingsViewModel::activate(&settings_view_model, behaviors);

        let reload_sender_for_action = reload_sender.clone();
        let update_use_system_theme_sender_for_action = update_use_system_theme_sender.clone();
        let switch_theme_mode_sender_for_action = switch_theme_mode_sender.clone();
        let update_use_primary_color_sender_for_action = update_use_primary_color_sender.clone();
        let update_use_secondary_color_sender_for_action =
            update_use_secondary_color_sender.clone();
        let update_use_tertiary_color_sender_for_action = update_use_tertiary_color_sender.clone();
        let update_primary_color_hex_sender_for_action = update_primary_color_hex_sender.clone();
        let update_secondary_color_hex_sender_for_action =
            update_secondary_color_hex_sender.clone();
        let update_tertiary_color_hex_sender_for_action = update_tertiary_color_hex_sender.clone();
        let actions = SettingsViewModelActions {
            reload: Some(Rc::new(move |_view_model| {
                let _ = reload_sender_for_action.send(ReloadSettingsCommand);
            })),
            update_use_system_theme: Some(Rc::new(move |_view_model, enabled| {
                let _ = update_use_system_theme_sender_for_action
                    .send(UpdateUseSystemThemeCommand { enabled });
            })),
            update_is_dark_mode: Some(Rc::new(move |_view_model, enabled| {
                let _ = switch_theme_mode_sender_for_action
                    .send(SwitchThemeModeCommand { is_dark: enabled });
            })),
            update_use_primary_color: Some(Rc::new(move |_view_model, enabled| {
                let _ = update_use_primary_color_sender_for_action
                    .send(UpdateUsePrimaryColorCommand { enabled });
            })),
            update_use_secondary_color: Some(Rc::new(move |_view_model, enabled| {
                let _ = update_use_secondary_color_sender_for_action
                    .send(UpdateUseSecondaryColorCommand { enabled });
            })),
            update_use_tertiary_color: Some(Rc::new(move |_view_model, enabled| {
                let _ = update_use_tertiary_color_sender_for_action
                    .send(UpdateUseTertiaryColorCommand { enabled });
            })),
            update_primary_color_hex: Some(Rc::new(move |_view_model, value| {
                let _ = update_primary_color_hex_sender_for_action
                    .send(UpdatePrimaryColorHexCommand { value });
            })),
            update_secondary_color_hex: Some(Rc::new(move |_view_model, value| {
                let _ = update_secondary_color_hex_sender_for_action
                    .send(UpdateSecondaryColorHexCommand { value });
            })),
            update_tertiary_color_hex: Some(Rc::new(move |_view_model, value| {
                let _ = update_tertiary_color_hex_sender_for_action
                    .send(UpdateTertiaryColorHexCommand { value });
            })),
        };

        settings_view_model.borrow_mut().set_actions(actions);

        Self {
            settings_view_model,
            _reload_sender: reload_sender,
        }
    }

    pub fn settings_view_model(&self) -> Rc<RefCell<SettingsViewModel>> {
        Rc::clone(&self.settings_view_model)
    }
}
