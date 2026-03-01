use std::collections::BTreeMap;

pub const THEME_KEY: &str = "theme";
pub const USE_SYSTEM_THEME_KEY: &str = "use_system_theme";
pub const USE_PRIMARY_COLOR_KEY: &str = "use_primary_color";
pub const USE_SECONDARY_COLOR_KEY: &str = "use_secondary_color";
pub const USE_TERTIARY_COLOR_KEY: &str = "use_tertiary_color";
pub const PRIMARY_COLOR_KEY: &str = "primary_color";
pub const SECONDARY_COLOR_KEY: &str = "secondary_color";
pub const TERTIARY_COLOR_KEY: &str = "tertiary_color";
pub const AUDIO_PLAYER_BACKEND_KEY: &str = "audio_player_backend";

pub const DEFAULT_PRIMARY_COLOR_HEX: &str = "#FF1976D2";
pub const DEFAULT_SECONDARY_COLOR_HEX: &str = "#FF675A84";
pub const DEFAULT_TERTIARY_COLOR_HEX: &str = "#FF825A2C";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ThemeMode {
    #[default]
    Light,
    Dark,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AudioPlayerBackend {
    #[default]
    Rodio,
    Awedio,
}

impl AudioPlayerBackend {
    #[must_use]
    pub fn as_preference(self) -> &'static str {
        match self {
            Self::Rodio => "rodio",
            Self::Awedio => "awedio",
        }
    }

    #[must_use]
    pub fn from_preference(value: &str) -> Self {
        if value.eq_ignore_ascii_case("awedio") {
            return Self::Awedio;
        }
        Self::Rodio
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MaterialSchemeState {
    pub light_background_hex: String,
    pub dark_background_hex: String,
}

impl Default for MaterialSchemeState {
    fn default() -> Self {
        Self {
            light_background_hex: "#FFFFFBFF".to_string(),
            dark_background_hex: "#FF131318".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SettingsState {
    pub theme_mode: ThemeMode,
    pub use_system_theme: bool,
    pub use_primary_color: bool,
    pub use_secondary_color: bool,
    pub use_tertiary_color: bool,
    pub primary_color_hex: String,
    pub secondary_color_hex: String,
    pub tertiary_color_hex: String,
    pub audio_player_backend: AudioPlayerBackend,
    pub preferences: BTreeMap<String, String>,
    pub material_scheme: MaterialSchemeState,
    pub material_update_count: usize,
}

impl Default for SettingsState {
    fn default() -> Self {
        Self {
            theme_mode: ThemeMode::Light,
            use_system_theme: true,
            use_primary_color: false,
            use_secondary_color: false,
            use_tertiary_color: false,
            primary_color_hex: DEFAULT_PRIMARY_COLOR_HEX.to_string(),
            secondary_color_hex: DEFAULT_SECONDARY_COLOR_HEX.to_string(),
            tertiary_color_hex: DEFAULT_TERTIARY_COLOR_HEX.to_string(),
            audio_player_backend: AudioPlayerBackend::Rodio,
            preferences: BTreeMap::new(),
            material_scheme: MaterialSchemeState::default(),
            material_update_count: 0,
        }
    }
}
