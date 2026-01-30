#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReloadSettingsCommand;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UpdateUseSystemThemeCommand {
    pub enabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SwitchThemeModeCommand {
    pub is_dark: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UpdateUsePrimaryColorCommand {
    pub enabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UpdateUseSecondaryColorCommand {
    pub enabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UpdateUseTertiaryColorCommand {
    pub enabled: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UpdatePrimaryColorHexCommand {
    pub value: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UpdateSecondaryColorHexCommand {
    pub value: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UpdateTertiaryColorHexCommand {
    pub value: String,
}
