use crate::nav_bar::nav_bar_component::actions::NavBarAction;
use crate::nav_bar::nav_bar_component::reducer::NavBarEffect;
use crate::nav_bar::nav_bar_component::reducer::reduce;
use crate::nav_bar::nav_bar_component::state::InteractionMode;
use crate::nav_bar::nav_bar_component::state::NavBarState;

#[test]
fn nav_bar_emits_command_effects_for_open_and_reset_actions() {
    let mut state = NavBarState::default();

    macro_rules! check {
        ($errors:expr, $cond:expr) => {
            if !$cond {
                $errors.push(format!("assertion failed: {}", stringify!($cond)));
            }
        };
    }

    macro_rules! check_eq {
        ($errors:expr, $left:expr, $right:expr) => {
            if $left != $right {
                $errors.push(format!(
                    "{} != {} (left = {:?}, right = {:?})",
                    stringify!($left),
                    stringify!($right),
                    $left,
                    $right
                ));
            }
        };
    }

    let mut errors = Vec::new();

    check!(errors, reduce(&mut state, NavBarAction::Initialize).is_empty());
    check_eq!(
        errors,
        reduce(&mut state, NavBarAction::OpenAudio),
        vec![NavBarEffect::OpenAudioRequested]
    );
    check_eq!(
        errors,
        reduce(&mut state, NavBarAction::OpenImage),
        vec![NavBarEffect::OpenImageRequested]
    );
    check_eq!(
        errors,
        reduce(&mut state, NavBarAction::ResetFloorViewport),
        vec![NavBarEffect::ResetFloorViewportRequested]
    );

    assert!(
        errors.is_empty(),
        "Assertion failures:\n{}",
        errors.join("\n")
    );
}

#[test]
fn nav_bar_mode_change_updates_state_and_emits_effect() {
    let mut state = NavBarState::default();

    macro_rules! check_eq {
        ($errors:expr, $left:expr, $right:expr) => {
            if $left != $right {
                $errors.push(format!(
                    "{} != {} (left = {:?}, right = {:?})",
                    stringify!($left),
                    stringify!($right),
                    $left,
                    $right
                ));
            }
        };
    }

    let effects = reduce(
        &mut state,
        NavBarAction::SetSelectedMode {
            mode: InteractionMode::Scale,
        },
    );

    let mut errors = Vec::new();

    check_eq!(errors, state.selected_mode, InteractionMode::Scale);
    check_eq!(
        errors,
        effects,
        vec![NavBarEffect::InteractionModeChanged {
            mode: InteractionMode::Scale
        }]
    );

    assert!(
        errors.is_empty(),
        "Assertion failures:\n{}",
        errors.join("\n")
    );
}
