use crate::drawer_host::actions::DrawerHostAction;
use crate::drawer_host::reducer::reduce;
use crate::drawer_host::state::DrawerHostState;
use crate::drawer_host::ui::inline_left_width;
use crate::drawer_host::ui::overlay_visible;

#[test]
fn inline_left_width_matches_open_state() {
    let closed_inline = DrawerHostState {
        inline_left: true,
        is_left_open: false,
        left_drawer_width: 320.0,
        ..DrawerHostState::default()
    };
    assert_eq!(inline_left_width(&closed_inline), 0.0);

    let open_inline = DrawerHostState {
        inline_left: true,
        is_left_open: true,
        left_drawer_width: 320.0,
        ..DrawerHostState::default()
    };
    assert_eq!(inline_left_width(&open_inline), 320.0);
}

#[test]
fn overlay_visibility_respects_open_drawers_and_click_away_flags() {
    let hidden = DrawerHostState::default();
    assert!(!overlay_visible(&hidden));

    let left_without_click_away = DrawerHostState {
        is_left_open: true,
        left_close_on_click_away: false,
        ..DrawerHostState::default()
    };
    assert!(!overlay_visible(&left_without_click_away));

    let right_with_click_away = DrawerHostState {
        is_right_open: true,
        right_close_on_click_away: true,
        ..DrawerHostState::default()
    };
    assert!(overlay_visible(&right_with_click_away));
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

    reduce(&mut state, DrawerHostAction::OverlayClicked);

    assert!(state.is_left_open);
    assert!(!state.is_right_open);
    assert!(state.is_top_open);
    assert!(!state.is_bottom_open);
}
