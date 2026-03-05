use egui::Rect;
use egui::pos2;
use egui::vec2;

use crate::nav_bar::nav_bar_component::hamburger_toggle_button::desired_size;
use crate::nav_bar::nav_bar_component::hamburger_toggle_button::geometry_for_rect;
use crate::nav_bar::nav_bar_component::hamburger_toggle_button::geometry_for_rect_with_progress;
use crate::nav_bar::nav_bar_component::hamburger_toggle_button::toggled_state_after_click;

#[test]
fn unchecked_geometry_keeps_three_parallel_bars() {
    let rect = Rect::from_min_size(pos2(0.0, 0.0), vec2(48.0, 48.0));
    let geometry = geometry_for_rect(rect, false);

    assert!(geometry.top_start.y < geometry.middle_start.y);
    assert!(geometry.middle_start.y < geometry.bottom_start.y);
    assert_eq!(geometry.top_start.y, geometry.top_end.y);
    assert_eq!(geometry.middle_start.y, geometry.middle_end.y);
    assert_eq!(geometry.bottom_start.y, geometry.bottom_end.y);
    assert_eq!(geometry.top_start.x, geometry.middle_start.x);
    assert_eq!(geometry.middle_start.x, geometry.bottom_start.x);
}

#[test]
fn checked_geometry_collapses_top_and_bottom_to_middle_with_rotations() {
    let rect = Rect::from_min_size(pos2(0.0, 0.0), vec2(48.0, 48.0));
    let geometry = geometry_for_rect(rect, true);

    assert_eq!(geometry.top_start.y, geometry.middle_start.y);
    assert_eq!(geometry.bottom_start.y, geometry.middle_start.y);

    let top_delta_y = geometry.top_end.y - geometry.top_start.y;
    let bottom_delta_y = geometry.bottom_end.y - geometry.bottom_start.y;
    assert!(top_delta_y < 0.0);
    assert!(bottom_delta_y > 0.0);

    let middle_delta_y = geometry.middle_end.y - geometry.middle_start.y;
    assert_eq!(middle_delta_y, 0.0);
}

#[test]
fn partial_transition_geometry_interpolates_between_unchecked_and_checked() {
    let rect = Rect::from_min_size(pos2(0.0, 0.0), vec2(48.0, 48.0));
    let unchecked = geometry_for_rect(rect, false);
    let checked = geometry_for_rect(rect, true);
    let halfway = geometry_for_rect_with_progress(rect, 0.5);

    assert!(halfway.top_start.y > unchecked.top_start.y);
    assert!(halfway.top_start.y < checked.top_start.y);
    assert!(halfway.bottom_start.y < unchecked.bottom_start.y);
    assert!(halfway.bottom_start.y > checked.bottom_start.y);
    assert!(halfway.top_end.y < unchecked.top_end.y);
    assert!(halfway.top_end.y > checked.top_end.y);
}

#[test]
fn desired_size_defaults_to_slint_minimum() {
    let size = desired_size(None);
    assert_eq!(size.x, 40.0);
    assert_eq!(size.y, 40.0);
}

#[test]
fn desired_size_clamps_to_slint_minimum_when_too_small() {
    let size = desired_size(Some(vec2(24.0, 30.0)));
    assert_eq!(size.x, 40.0);
    assert_eq!(size.y, 40.0);
}

#[test]
fn toggle_on_click_matches_slint_toggle_semantics() {
    assert!(toggled_state_after_click(false, true, true));
    assert!(!toggled_state_after_click(false, false, true));
    assert!(!toggled_state_after_click(false, true, false));
}
