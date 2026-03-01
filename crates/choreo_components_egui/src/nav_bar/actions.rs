use super::state::InteractionMode;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NavBarAction {
    Initialize,
    SetModeSelectionEnabled {
        enabled: bool,
    },
    ToggleNavigation,
    CloseNavigation,
    ToggleChoreographySettings,
    CloseChoreographySettings,
    SetAudioPlayerOpened {
        is_open: bool,
    },
    SetSelectedMode {
        mode: InteractionMode,
    },
    OpenAudio,
    OpenImage,
    ResetFloorViewport,
}
