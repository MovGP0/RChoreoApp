use choreo_master_mobile_json::Color;

use crate::behavior::{Behavior, CompositeDisposable};
use nject::injectable;

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

#[injectable]
#[inject(|behaviors: Vec<Box<dyn Behavior<SettingsViewModel>>>| Self::new(behaviors))]
pub struct SettingsViewModel {
    pub theme_mode: ThemeMode,
    pub use_system_theme: bool,
    pub use_primary_color: bool,
    pub use_secondary_color: bool,
    pub use_tertiary_color: bool,
    pub primary_color: Color,
    pub secondary_color: Color,
    pub tertiary_color: Color,
    disposables: CompositeDisposable,
}

impl SettingsViewModel {
    pub fn new(behaviors: Vec<Box<dyn Behavior<SettingsViewModel>>>) -> Self {
        let mut view_model = Self {
            theme_mode: ThemeMode::Light,
            use_system_theme: false,
            use_primary_color: false,
            use_secondary_color: false,
            use_tertiary_color: false,
            primary_color: default_primary_color(),
            secondary_color: default_secondary_color(),
            tertiary_color: default_tertiary_color(),
            disposables: CompositeDisposable::new(),
        };

        let mut disposables = CompositeDisposable::new();
        for behavior in behaviors.iter() {
            behavior.activate(&mut view_model, &mut disposables);
        }

        view_model.disposables = disposables;
        view_model
    }

    pub fn dispose(&mut self) {
        self.disposables.dispose_all();
    }
}

impl Default for SettingsViewModel {
    fn default() -> Self {
        Self::new(Vec::new())
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
