use std::collections::BTreeMap;

use crate::audio_player::AudioPlayerBackend;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SettingsAction {
    Initialize,
    NavigateBack,
    LoadFromPreferences { entries: BTreeMap<String, String> },
    Reload,
    UpdateUseSystemTheme { enabled: bool },
    UpdateIsDarkMode { enabled: bool },
    UpdateUsePrimaryColor { enabled: bool },
    UpdateUseSecondaryColor { enabled: bool },
    UpdateUseTertiaryColor { enabled: bool },
    UpdatePrimaryColorHex { value: String },
    UpdateSecondaryColorHex { value: String },
    UpdateTertiaryColorHex { value: String },
    UpdateAudioPlayerBackend { backend: AudioPlayerBackend },
}
