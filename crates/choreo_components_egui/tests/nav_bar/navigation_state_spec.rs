use crate::nav_bar::nav_bar_component::actions::NavBarAction;
use crate::nav_bar::nav_bar_component::reducer::reduce;
use crate::nav_bar::nav_bar_component::state::NavBarState;

#[test]
fn navigation_toggles_width_and_open_state() {
    let mut state = NavBarState::default();

    reduce(&mut state, NavBarAction::ToggleNavigation);
    assert!(state.is_nav_open);
    assert_eq!(state.nav_width, NavBarState::DEFAULT_NAV_WIDTH);

    reduce(&mut state, NavBarAction::CloseNavigation);
    assert!(!state.is_nav_open);
    assert_eq!(state.nav_width, 0.0);
}

#[test]
fn settings_and_audio_state_changes_are_applied() {
    let mut state = NavBarState::default();

    reduce(&mut state, NavBarAction::ToggleChoreographySettings);
    assert!(state.is_choreography_settings_open);
    reduce(&mut state, NavBarAction::CloseChoreographySettings);
    assert!(!state.is_choreography_settings_open);

    reduce(&mut state, NavBarAction::SetAudioPlayerOpened { is_open: true });
    assert!(state.is_audio_player_open);
    reduce(&mut state, NavBarAction::SetAudioPlayerOpened { is_open: false });
    assert!(!state.is_audio_player_open);

    reduce(
        &mut state,
        NavBarAction::SetModeSelectionEnabled { enabled: false },
    );
    assert!(!state.is_mode_selection_enabled);
}
