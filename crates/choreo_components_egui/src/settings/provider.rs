use std::cell::RefCell;
use std::rc::Rc;

use super::actions::SettingsAction;
use super::messages::SwitchThemeModeCommand;
use super::messages::UpdateAudioPlayerBackendCommand;
use super::messages::UpdatePrimaryColorHexCommand;
use super::messages::UpdateSecondaryColorHexCommand;
use super::messages::UpdateTertiaryColorHexCommand;
use super::messages::UpdateUsePrimaryColorCommand;
use super::messages::UpdateUseSecondaryColorCommand;
use super::messages::UpdateUseSystemThemeCommand;
use super::messages::UpdateUseTertiaryColorCommand;
use super::reducer::reduce;
use super::view_model::SettingsViewModelActions;
use super::view_model::SettingsViewModel;

pub struct SettingsProvider {
    settings_view_model: Rc<RefCell<SettingsViewModel>>,
}

impl Default for SettingsProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl SettingsProvider {
    #[must_use]
    pub fn new() -> Self {
        let settings_view_model = Rc::new(RefCell::new(SettingsViewModel::new()));
        settings_view_model
            .borrow_mut()
            .set_self_handle(Rc::downgrade(&settings_view_model));

        let actions = SettingsViewModelActions {
            reload: Some(Rc::new(|view_model| {
                reduce(&mut view_model.state, SettingsAction::Reload);
            })),
            navigate_back: Some(Rc::new(|view_model| {
                reduce(&mut view_model.state, SettingsAction::NavigateBack);
            })),
            update_use_system_theme: Some(Rc::new(|view_model, command: UpdateUseSystemThemeCommand| {
                reduce(
                    &mut view_model.state,
                    SettingsAction::UpdateUseSystemTheme {
                        enabled: command.enabled,
                    },
                );
            })),
            update_is_dark_mode: Some(Rc::new(|view_model, command: SwitchThemeModeCommand| {
                reduce(
                    &mut view_model.state,
                    SettingsAction::UpdateIsDarkMode {
                        enabled: command.is_dark,
                    },
                );
            })),
            update_use_primary_color: Some(Rc::new(|view_model, command: UpdateUsePrimaryColorCommand| {
                reduce(
                    &mut view_model.state,
                    SettingsAction::UpdateUsePrimaryColor {
                        enabled: command.enabled,
                    },
                );
            })),
            update_use_secondary_color: Some(Rc::new(|view_model, command: UpdateUseSecondaryColorCommand| {
                reduce(
                    &mut view_model.state,
                    SettingsAction::UpdateUseSecondaryColor {
                        enabled: command.enabled,
                    },
                );
            })),
            update_use_tertiary_color: Some(Rc::new(|view_model, command: UpdateUseTertiaryColorCommand| {
                reduce(
                    &mut view_model.state,
                    SettingsAction::UpdateUseTertiaryColor {
                        enabled: command.enabled,
                    },
                );
            })),
            update_primary_color_hex: Some(Rc::new(|view_model, command: UpdatePrimaryColorHexCommand| {
                reduce(
                    &mut view_model.state,
                    SettingsAction::UpdatePrimaryColorHex {
                        value: command.value,
                    },
                );
            })),
            update_secondary_color_hex: Some(Rc::new(|view_model, command: UpdateSecondaryColorHexCommand| {
                reduce(
                    &mut view_model.state,
                    SettingsAction::UpdateSecondaryColorHex {
                        value: command.value,
                    },
                );
            })),
            update_tertiary_color_hex: Some(Rc::new(|view_model, command: UpdateTertiaryColorHexCommand| {
                reduce(
                    &mut view_model.state,
                    SettingsAction::UpdateTertiaryColorHex {
                        value: command.value,
                    },
                );
            })),
            update_audio_player_backend: Some(Rc::new(|view_model, command: UpdateAudioPlayerBackendCommand| {
                reduce(
                    &mut view_model.state,
                    SettingsAction::UpdateAudioPlayerBackend {
                        backend: command.backend,
                    },
                );
            })),
        };
        settings_view_model.borrow_mut().set_actions(actions);
        reduce(
            &mut settings_view_model.borrow_mut().state,
            SettingsAction::Initialize,
        );

        Self {
            settings_view_model,
        }
    }

    #[must_use]
    pub fn settings_view_model(&self) -> Rc<RefCell<SettingsViewModel>> {
        Rc::clone(&self.settings_view_model)
    }
}
