use crate::drawer_host::actions::DrawerHostAction;
use crate::drawer_host::reducer::reduce;
use crate::drawer_host::state::DrawerHostOpenMode;
use crate::drawer_host::state::DrawerHostState;
use crate::drawer_host::ui::inline_left_width;
use crate::drawer_host::ui::is_inline_left_layout;
use crate::drawer_host::ui::overlay_visible;

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

macro_rules! check {
    ($errors:expr, $condition:expr) => {
        if !$condition {
            $errors.push(format!("condition failed: {}", stringify!($condition)));
        }
    };
}

fn assert_no_errors(errors: Vec<String>) {
    assert!(
        errors.is_empty(),
        "Assertion failures:\n{}",
        errors.join("\n")
    );
}

#[test]
fn inline_left_width_matches_open_state() {
    let closed_inline = DrawerHostState {
        open_mode: DrawerHostOpenMode::Standard,
        is_left_open: false,
        left_drawer_width: 320.0,
        ..DrawerHostState::default()
    };
    let open_inline = DrawerHostState {
        open_mode: DrawerHostOpenMode::Standard,
        is_left_open: true,
        left_drawer_width: 320.0,
        ..DrawerHostState::default()
    };

    let mut errors = Vec::new();

    check_eq!(errors, inline_left_width(&closed_inline, 1280.0), 0.0);
    check_eq!(errors, inline_left_width(&open_inline, 1280.0), 320.0);

    assert_no_errors(errors);
}

#[test]
fn overlay_visibility_respects_open_drawers_and_click_away_flags() {
    let hidden = DrawerHostState::default();
    let left_without_click_away = DrawerHostState {
        is_left_open: true,
        left_close_on_click_away: false,
        ..DrawerHostState::default()
    };
    let right_with_click_away = DrawerHostState {
        is_right_open: true,
        right_close_on_click_away: true,
        ..DrawerHostState::default()
    };

    let mut errors = Vec::new();

    check!(errors, !overlay_visible(&hidden, 1280.0));
    check!(
        errors,
        !overlay_visible(&left_without_click_away, 1280.0)
    );
    check!(errors, overlay_visible(&right_with_click_away, 1280.0));

    assert_no_errors(errors);
}

#[test]
fn standard_mode_inlines_only_the_left_drawer_above_breakpoint() {
    let state = DrawerHostState {
        open_mode: DrawerHostOpenMode::Standard,
        is_left_open: true,
        ..DrawerHostState::default()
    };

    let mut errors = Vec::new();

    check!(errors, is_inline_left_layout(&state, 1280.0));
    check!(errors, !is_inline_left_layout(&state, 640.0));

    assert_no_errors(errors);
}

#[test]
fn modal_mode_keeps_left_drawer_overlay_even_above_breakpoint() {
    let state = DrawerHostState {
        open_mode: DrawerHostOpenMode::Modal,
        is_left_open: true,
        ..DrawerHostState::default()
    };

    let mut errors = Vec::new();

    check!(errors, !is_inline_left_layout(&state, 1280.0));
    check!(errors, overlay_visible(&state, 1280.0));

    assert_no_errors(errors);
}

#[test]
fn inline_left_drawer_does_not_gate_overlay_or_click_away_close() {
    let state = DrawerHostState {
        open_mode: DrawerHostOpenMode::Standard,
        is_left_open: true,
        is_right_open: true,
        ..DrawerHostState::default()
    };

    let mut reduced_state = state.clone();
    reduce(
        &mut reduced_state,
        DrawerHostAction::OverlayClicked {
            close_left: false,
            close_right: true,
            close_top: false,
            close_bottom: false,
        },
    );

    let mut errors = Vec::new();

    check!(errors, is_inline_left_layout(&state, 1280.0));
    check!(errors, overlay_visible(&state, 1280.0));
    check!(errors, reduced_state.is_left_open);
    check!(errors, !reduced_state.is_right_open);

    assert_no_errors(errors);
}

#[test]
fn overlay_clicked_closes_only_drawers_that_allow_click_away() {
    let mut state = DrawerHostState {
        is_left_open: true,
        is_right_open: true,
        is_top_open: true,
        is_bottom_open: true,
        left_close_on_click_away: false,
        right_close_on_click_away: true,
        top_close_on_click_away: false,
        bottom_close_on_click_away: true,
        ..DrawerHostState::default()
    };

    reduce(
        &mut state,
        DrawerHostAction::OverlayClicked {
            close_left: false,
            close_right: true,
            close_top: false,
            close_bottom: true,
        },
    );

    let mut errors = Vec::new();

    check!(errors, state.is_left_open);
    check!(errors, !state.is_right_open);
    check!(errors, state.is_top_open);
    check!(errors, !state.is_bottom_open);

    assert_no_errors(errors);
}
