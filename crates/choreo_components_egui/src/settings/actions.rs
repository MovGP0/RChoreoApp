use std::collections::BTreeMap;

use super::state::AudioPlayerBackend;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SettingsAction {
    Initialize,
    LoadFromPreferences {
        entries: BTreeMap<String, String>,
    },
    Reload,
    UpdateUseSystemTheme {
        enabled: bool,
    },
    UpdateIsDarkMode {
        enabled: bool,
    },
    UpdateUsePrimaryColor {
        enabled: bool,
    },
    UpdateUseSecondaryColor {
        enabled: bool,
    },
    UpdateUseTertiaryColor {
        enabled: bool,
    },
    UpdatePrimaryColorHex {
        value: String,
    },
    UpdateSecondaryColorHex {
        value: String,
    },
    UpdateTertiaryColorHex {
        value: String,
    },
    UpdateAudioPlayerBackend {
        backend: AudioPlayerBackend,
    },
}
