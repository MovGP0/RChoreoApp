use crate::audio_player::AudioPlayerBackend;
use crate::behavior::Behavior;
use crate::behavior::CompositeDisposable;
use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

use super::messages::SwitchThemeModeCommand;
use super::messages::UpdateAudioPlayerBackendCommand;
use super::messages::UpdatePrimaryColorHexCommand;
use super::messages::UpdateSecondaryColorHexCommand;
use super::messages::UpdateTertiaryColorHexCommand;
use super::messages::UpdateUsePrimaryColorCommand;
use super::messages::UpdateUseSecondaryColorCommand;
use super::messages::UpdateUseSystemThemeCommand;
use super::messages::UpdateUseTertiaryColorCommand;
use super::state::SettingsState;

type SettingsActionHandler = Rc<dyn Fn(&mut SettingsViewModel)>;
type SettingsBooleanHandler<C> = Rc<dyn Fn(&mut SettingsViewModel, C)>;
type SettingsStringHandler<C> = Rc<dyn Fn(&mut SettingsViewModel, C)>;

#[derive(Clone, Default)]
pub struct SettingsViewModelActions {
    pub reload: Option<SettingsActionHandler>,
    pub navigate_back: Option<SettingsActionHandler>,
    pub update_use_system_theme: Option<SettingsBooleanHandler<UpdateUseSystemThemeCommand>>,
    pub update_is_dark_mode: Option<SettingsBooleanHandler<SwitchThemeModeCommand>>,
    pub update_use_primary_color: Option<SettingsBooleanHandler<UpdateUsePrimaryColorCommand>>,
    pub update_use_secondary_color: Option<SettingsBooleanHandler<UpdateUseSecondaryColorCommand>>,
    pub update_use_tertiary_color: Option<SettingsBooleanHandler<UpdateUseTertiaryColorCommand>>,
    pub update_primary_color_hex: Option<SettingsStringHandler<UpdatePrimaryColorHexCommand>>,
    pub update_secondary_color_hex: Option<SettingsStringHandler<UpdateSecondaryColorHexCommand>>,
    pub update_tertiary_color_hex: Option<SettingsStringHandler<UpdateTertiaryColorHexCommand>>,
    pub update_audio_player_backend:
        Option<SettingsBooleanHandler<UpdateAudioPlayerBackendCommand>>,
}

pub struct SettingsViewModel {
    pub state: SettingsState,
    actions: SettingsViewModelActions,
    disposables: CompositeDisposable,
    self_handle: Option<Weak<RefCell<SettingsViewModel>>>,
}

impl Default for SettingsViewModel {
    fn default() -> Self {
        Self::new()
    }
}

impl SettingsViewModel {
    #[must_use]
    pub fn new() -> Self {
        Self {
            state: SettingsState::default(),
            actions: SettingsViewModelActions::default(),
            disposables: CompositeDisposable::new(),
            self_handle: None,
        }
    }

    pub fn activate(
        view_model: &Rc<RefCell<SettingsViewModel>>,
        behaviors: Vec<Box<dyn Behavior<SettingsViewModel>>>,
    ) {
        let mut disposables = CompositeDisposable::new();
        {
            let mut view_model = view_model.borrow_mut();
            for behavior in behaviors {
                behavior.activate(&mut view_model, &mut disposables);
            }
        }
        view_model.borrow_mut().disposables = disposables;
    }

    pub fn set_actions(&mut self, actions: SettingsViewModelActions) {
        self.actions = actions;
    }

    pub fn set_self_handle(&mut self, handle: Weak<RefCell<SettingsViewModel>>) {
        self.self_handle = Some(handle);
    }

    #[must_use]
    pub fn self_handle(&self) -> Option<Weak<RefCell<SettingsViewModel>>> {
        self.self_handle.clone()
    }

    pub fn reload(&mut self) {
        if let Some(handler) = self.actions.reload.clone() {
            handler(self);
        }
    }

    pub fn navigate_back(&mut self) {
        if let Some(handler) = self.actions.navigate_back.clone() {
            handler(self);
        }
    }

    pub fn update_use_system_theme(&mut self, enabled: bool) {
        if let Some(handler) = self.actions.update_use_system_theme.clone() {
            handler(self, UpdateUseSystemThemeCommand { enabled });
        }
    }

    pub fn update_is_dark_mode(&mut self, enabled: bool) {
        if let Some(handler) = self.actions.update_is_dark_mode.clone() {
            handler(self, SwitchThemeModeCommand { is_dark: enabled });
        }
    }

    pub fn update_use_primary_color(&mut self, enabled: bool) {
        if let Some(handler) = self.actions.update_use_primary_color.clone() {
            handler(self, UpdateUsePrimaryColorCommand { enabled });
        }
    }

    pub fn update_use_secondary_color(&mut self, enabled: bool) {
        if let Some(handler) = self.actions.update_use_secondary_color.clone() {
            handler(self, UpdateUseSecondaryColorCommand { enabled });
        }
    }

    pub fn update_use_tertiary_color(&mut self, enabled: bool) {
        if let Some(handler) = self.actions.update_use_tertiary_color.clone() {
            handler(self, UpdateUseTertiaryColorCommand { enabled });
        }
    }

    pub fn update_primary_color_hex(&mut self, value: String) {
        if let Some(handler) = self.actions.update_primary_color_hex.clone() {
            handler(self, UpdatePrimaryColorHexCommand { value });
        }
    }

    pub fn update_secondary_color_hex(&mut self, value: String) {
        if let Some(handler) = self.actions.update_secondary_color_hex.clone() {
            handler(self, UpdateSecondaryColorHexCommand { value });
        }
    }

    pub fn update_tertiary_color_hex(&mut self, value: String) {
        if let Some(handler) = self.actions.update_tertiary_color_hex.clone() {
            handler(self, UpdateTertiaryColorHexCommand { value });
        }
    }

    pub fn update_audio_player_backend(&mut self, backend: AudioPlayerBackend) {
        if let Some(handler) = self.actions.update_audio_player_backend.clone() {
            handler(self, UpdateAudioPlayerBackendCommand { backend });
        }
    }
}

impl Drop for SettingsViewModel {
    fn drop(&mut self) {
        self.disposables.dispose_all();
    }
}
