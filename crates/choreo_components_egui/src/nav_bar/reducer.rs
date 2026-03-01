use super::actions::NavBarAction;
use super::state::InteractionMode;
use super::state::NavBarState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NavBarEffect {
    OpenAudioRequested,
    OpenImageRequested,
    ResetFloorViewportRequested,
    InteractionModeChanged {
        mode: InteractionMode,
    },
}

pub fn reduce(state: &mut NavBarState, action: NavBarAction) -> Vec<NavBarEffect> {
    match action {
        NavBarAction::Initialize => Vec::new(),
        NavBarAction::SetModeSelectionEnabled { enabled } => {
            state.is_mode_selection_enabled = enabled;
            Vec::new()
        }
        NavBarAction::ToggleNavigation => {
            state.is_nav_open = !state.is_nav_open;
            state.nav_width = if state.is_nav_open {
                NavBarState::DEFAULT_NAV_WIDTH
            } else {
                0.0
            };
            Vec::new()
        }
        NavBarAction::CloseNavigation => {
            state.is_nav_open = false;
            state.nav_width = 0.0;
            Vec::new()
        }
        NavBarAction::ToggleChoreographySettings => {
            state.is_choreography_settings_open = !state.is_choreography_settings_open;
            Vec::new()
        }
        NavBarAction::CloseChoreographySettings => {
            state.is_choreography_settings_open = false;
            Vec::new()
        }
        NavBarAction::SetAudioPlayerOpened { is_open } => {
            state.is_audio_player_open = is_open;
            Vec::new()
        }
        NavBarAction::SetSelectedMode { mode } => {
            state.selected_mode = mode;
            vec![NavBarEffect::InteractionModeChanged { mode }]
        }
        NavBarAction::OpenAudio => {
            vec![NavBarEffect::OpenAudioRequested]
        }
        NavBarAction::OpenImage => {
            vec![NavBarEffect::OpenImageRequested]
        }
        NavBarAction::ResetFloorViewport => {
            vec![NavBarEffect::ResetFloorViewportRequested]
        }
    }
}
