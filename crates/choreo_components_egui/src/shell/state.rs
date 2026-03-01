#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ShellThemeMode {
    #[default]
    Light,
    Dark,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShellMaterialSchemes {
    pub light_background_hex: String,
    pub dark_background_hex: String,
}

impl Default for ShellMaterialSchemes {
    fn default() -> Self {
        Self {
            light_background_hex: "#FFFFFBFF".to_string(),
            dark_background_hex: "#FF131318".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShellState {
    pub app_title: String,
    pub theme_mode: ShellThemeMode,
    pub schemes: ShellMaterialSchemes,
    pub active_background_hex: String,
}

impl Default for ShellState {
    fn default() -> Self {
        let schemes = ShellMaterialSchemes::default();
        Self {
            app_title: "ChoreoApp".to_string(),
            theme_mode: ShellThemeMode::Light,
            active_background_hex: schemes.light_background_hex.clone(),
            schemes,
        }
    }
}
