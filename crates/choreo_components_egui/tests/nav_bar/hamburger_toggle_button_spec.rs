use egui::Rect;
use egui::pos2;
use egui::vec2;

use crate::nav_bar::nav_bar_component::hamburger_toggle_button::geometry_for_rect;

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
