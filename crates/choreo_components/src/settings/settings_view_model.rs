use choreo_master_mobile_json::Color;
use nject::injectable;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

use crate::behavior::{Behavior, CompositeDisposable};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeMode {
    Light,
    Dark,
}

pub struct BooleanNegationConverter;

impl BooleanNegationConverter {
    pub fn convert(value: bool) -> bool {
        !value
    }
}

type SettingsActionHandler = Rc<dyn Fn(&mut SettingsViewModel)>;
type SettingsBooleanHandler = Rc<dyn Fn(&mut SettingsViewModel, bool)>;
type SettingsStringHandler = Rc<dyn Fn(&mut SettingsViewModel, String)>;

#[derive(Clone, Default)]
pub struct SettingsViewModelActions {
    pub reload: Option<SettingsActionHandler>,
    pub update_use_system_theme: Option<SettingsBooleanHandler>,
    pub update_is_dark_mode: Option<SettingsBooleanHandler>,
    pub update_use_primary_color: Option<SettingsBooleanHandler>,
    pub update_use_secondary_color: Option<SettingsBooleanHandler>,
    pub update_use_tertiary_color: Option<SettingsBooleanHandler>,
    pub update_primary_color_hex: Option<SettingsStringHandler>,
    pub update_secondary_color_hex: Option<SettingsStringHandler>,
    pub update_tertiary_color_hex: Option<SettingsStringHandler>,
}

#[injectable]
#[inject(|| Self::new())]
pub struct SettingsViewModel {
    actions: SettingsViewModelActions,
    pub theme_mode: ThemeMode,
    pub use_system_theme: bool,
    pub use_primary_color: bool,
    pub use_secondary_color: bool,
    pub use_tertiary_color: bool,
    pub primary_color: Color,
    pub secondary_color: Color,
    pub tertiary_color: Color,
    disposables: CompositeDisposable,
    self_handle: Option<Weak<RefCell<SettingsViewModel>>>,
}

impl SettingsViewModel {
    pub fn new() -> Self {
        Self {
            actions: SettingsViewModelActions::default(),
            theme_mode: ThemeMode::Light,
            use_system_theme: false,
            use_primary_color: false,
            use_secondary_color: false,
            use_tertiary_color: false,
            primary_color: default_primary_color(),
            secondary_color: default_secondary_color(),
            tertiary_color: default_tertiary_color(),
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
            for behavior in behaviors.iter() {
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

    pub fn self_handle(&self) -> Option<Weak<RefCell<SettingsViewModel>>> {
        self.self_handle.clone()
    }

    pub fn reload(&mut self) {
        if let Some(handler) = self.actions.reload.clone() {
            handler(self);
        }
    }

    pub fn update_use_system_theme(&mut self, enabled: bool) {
        if let Some(handler) = self.actions.update_use_system_theme.clone() {
            handler(self, enabled);
        }
    }

    pub fn update_is_dark_mode(&mut self, enabled: bool) {
        if let Some(handler) = self.actions.update_is_dark_mode.clone() {
            handler(self, enabled);
        }
    }

    pub fn update_use_primary_color(&mut self, enabled: bool) {
        if let Some(handler) = self.actions.update_use_primary_color.clone() {
            handler(self, enabled);
        }
    }

    pub fn update_use_secondary_color(&mut self, enabled: bool) {
        if let Some(handler) = self.actions.update_use_secondary_color.clone() {
            handler(self, enabled);
        }
    }

    pub fn update_use_tertiary_color(&mut self, enabled: bool) {
        if let Some(handler) = self.actions.update_use_tertiary_color.clone() {
            handler(self, enabled);
        }
    }

    pub fn update_primary_color_hex(&mut self, value: String) {
        if let Some(handler) = self.actions.update_primary_color_hex.clone() {
            handler(self, value);
        }
    }

    pub fn update_secondary_color_hex(&mut self, value: String) {
        if let Some(handler) = self.actions.update_secondary_color_hex.clone() {
            handler(self, value);
        }
    }

    pub fn update_tertiary_color_hex(&mut self, value: String) {
        if let Some(handler) = self.actions.update_tertiary_color_hex.clone() {
            handler(self, value);
        }
    }
}

impl Drop for SettingsViewModel {
    fn drop(&mut self) {
        self.disposables.dispose_all();
    }
}

impl Default for SettingsViewModel {
    fn default() -> Self {
        Self::new()
    }
}

pub fn default_primary_color() -> Color {
    Color {
        a: 255,
        r: 0x19,
        g: 0x76,
        b: 0xD2,
    }
}

pub fn default_secondary_color() -> Color {
    Color {
        a: 255,
        r: 0x67,
        g: 0x5A,
        b: 0x84,
    }
}

pub fn default_tertiary_color() -> Color {
    Color {
        a: 255,
        r: 0x82,
        g: 0x5A,
        b: 0x2C,
    }
}
