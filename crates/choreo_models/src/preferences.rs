pub struct SettingsPreferenceKeys;

impl SettingsPreferenceKeys {
    pub const THEME: &str = "Theme";
    pub const USE_SYSTEM_THEME: &str = "UseSystemTheme";
    pub const USE_PRIMARY_COLOR: &str = "UsePrimaryColor";
    pub const USE_SECONDARY_COLOR: &str = "UseSecondaryColor";
    pub const USE_TERTIARY_COLOR: &str = "UseTertiaryColor";
    pub const PRIMARY_COLOR: &str = "PrimaryColor";
    pub const SECONDARY_COLOR: &str = "SecondaryColor";
    pub const TERTIARY_COLOR: &str = "TertiaryColor";
    pub const LAST_OPENED_CHOREO_FILE: &str = "LastOpenedChoreoFile";
    pub const LAST_OPENED_AUDIO_FILE: &str = "LastOpenedAudioFile";
    pub const AUDIO_PLAYER_BACKEND: &str = "AudioPlayerBackend";
    pub const LAST_OPENED_SVG_FILE: &str = "LastOpenedSvgFile";
    pub const DRAW_PATH_FROM: &str = "DrawPathFrom";
    pub const DRAW_PATH_TO: &str = "DrawPathTo";
    pub const POSITIONS_AT_SIDE: &str = "PositionsAtSide";
    pub const SHOW_TIMESTAMPS: &str = "ShowTimestamps";
    pub const SNAP_TO_GRID: &str = "SnapToGrid";
    pub const SHOW_LEGEND: &str = "ShowLegend";
}
