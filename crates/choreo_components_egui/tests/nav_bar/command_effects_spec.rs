use crate::nav_bar::nav_bar_component::actions::NavBarAction;
use crate::nav_bar::nav_bar_component::reducer::NavBarEffect;
use crate::nav_bar::nav_bar_component::reducer::reduce;
use crate::nav_bar::nav_bar_component::state::InteractionMode;
use crate::nav_bar::nav_bar_component::state::NavBarState;

#[test]
fn nav_bar_emits_command_effects_for_open_and_reset_actions() {
    let mut state = NavBarState::default();

    assert!(reduce(&mut state, NavBarAction::Initialize).is_empty());
    assert_eq!(
        reduce(&mut state, NavBarAction::OpenAudio),
        vec![NavBarEffect::OpenAudioRequested]
    );
    assert_eq!(
        reduce(&mut state, NavBarAction::OpenImage),
        vec![NavBarEffect::OpenImageRequested]
    );
    assert_eq!(
        reduce(&mut state, NavBarAction::ResetFloorViewport),
        vec![NavBarEffect::ResetFloorViewportRequested]
    );
}

#[test]
fn nav_bar_mode_change_updates_state_and_emits_effect() {
    let mut state = NavBarState::default();

    let effects = reduce(
        &mut state,
        NavBarAction::SetSelectedMode {
            mode: InteractionMode::Scale,
        },
    );

    assert_eq!(state.selected_mode, InteractionMode::Scale);
    assert_eq!(
        effects,
        vec![NavBarEffect::InteractionModeChanged {
            mode: InteractionMode::Scale
        }]
    );
}
