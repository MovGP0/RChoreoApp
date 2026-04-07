use crate::nav_bar::nav_bar_component::actions::NavBarAction;
use crate::nav_bar::nav_bar_component::reducer::reduce;
use crate::nav_bar::nav_bar_component::state::NavBarState;

#[test]
fn navigation_toggles_width_and_open_state() {
    let mut state = NavBarState::default();
    let mut errors = Vec::new();

    macro_rules! check {
        ($errors:expr, $cond:expr) => {
            if !$cond {
                $errors.push(format!("Assertion failed: {}", stringify!($cond)));
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

    reduce(&mut state, NavBarAction::ToggleNavigation);
    check!(errors, state.is_nav_open);
    check_eq!(errors, state.nav_width, NavBarState::DEFAULT_NAV_WIDTH);

    reduce(&mut state, NavBarAction::CloseNavigation);
    check!(errors, !state.is_nav_open);
    check_eq!(errors, state.nav_width, 0.0);

    assert!(
        errors.is_empty(),
        "Assertion failures:\n{}",
        errors.join("\n")
    );
}

#[test]
fn settings_and_audio_state_changes_are_applied() {
    let mut state = NavBarState::default();
    let mut errors = Vec::new();

    macro_rules! check {
        ($errors:expr, $cond:expr) => {
            if !$cond {
                $errors.push(format!("Assertion failed: {}", stringify!($cond)));
            }
        };
    }

    reduce(&mut state, NavBarAction::ToggleChoreographySettings);
    check!(errors, state.is_choreography_settings_open);
    reduce(&mut state, NavBarAction::CloseChoreographySettings);
    check!(errors, !state.is_choreography_settings_open);

    reduce(
        &mut state,
        NavBarAction::SetAudioPlayerOpened { is_open: true },
    );
    check!(errors, state.is_audio_player_open);
    reduce(
        &mut state,
        NavBarAction::SetAudioPlayerOpened { is_open: false },
    );
    check!(errors, !state.is_audio_player_open);

    reduce(
        &mut state,
        NavBarAction::SetFloorSvgOverlayOpened { is_open: true },
    );
    check!(errors, state.is_floor_svg_overlay_open);
    reduce(
        &mut state,
        NavBarAction::SetFloorSvgOverlayOpened { is_open: false },
    );
    check!(errors, !state.is_floor_svg_overlay_open);

    reduce(
        &mut state,
        NavBarAction::SetModeSelectionEnabled { enabled: false },
    );
    check!(errors, !state.is_mode_selection_enabled);

    assert!(
        errors.is_empty(),
        "Assertion failures:\n{}",
        errors.join("\n")
    );
}
