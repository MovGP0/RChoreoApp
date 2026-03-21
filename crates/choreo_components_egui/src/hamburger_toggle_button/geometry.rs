use egui::Pos2;
use egui::Rect;
use egui::Vec2;
use egui::vec2;

use super::tokens::content_padding_token;
use super::tokens::minimum_button_size_token;

const CHECKED_ROTATION_DEGREES: f32 = 35.0;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HamburgerToggleButtonGeometry {
    pub top_start: Pos2,
    pub top_end: Pos2,
    pub middle_start: Pos2,
    pub middle_end: Pos2,
    pub bottom_start: Pos2,
    pub bottom_end: Pos2,
    pub thickness: f32,
}

#[must_use]
pub fn desired_size(size: Option<Vec2>) -> Vec2 {
    let minimum_size = minimum_button_size_token();
    let requested_size = size.unwrap_or(vec2(minimum_size, minimum_size));
    vec2(
        requested_size.x.max(minimum_size),
        requested_size.y.max(minimum_size),
    )
}

#[must_use]
pub fn geometry_for_rect(rect: Rect, checked: bool) -> HamburgerToggleButtonGeometry {
    let checked_progress = if checked { 1.0 } else { 0.0 };
    geometry_for_rect_with_progress(rect, checked_progress)
}

#[must_use]
pub fn geometry_for_rect_with_progress(
    rect: Rect,
    checked_progress: f32,
) -> HamburgerToggleButtonGeometry {
    let checked_progress = checked_progress.clamp(0.0, 1.0);
    let unchecked = unchecked_geometry_for_rect(rect);
    let checked = checked_geometry_for_rect(rect);

    HamburgerToggleButtonGeometry {
        top_start: lerp_pos2(unchecked.top_start, checked.top_start, checked_progress),
        top_end: lerp_pos2(unchecked.top_end, checked.top_end, checked_progress),
        middle_start: lerp_pos2(
            unchecked.middle_start,
            checked.middle_start,
            checked_progress,
        ),
        middle_end: lerp_pos2(unchecked.middle_end, checked.middle_end, checked_progress),
        bottom_start: lerp_pos2(
            unchecked.bottom_start,
            checked.bottom_start,
            checked_progress,
        ),
        bottom_end: lerp_pos2(unchecked.bottom_end, checked.bottom_end, checked_progress),
        thickness: egui::lerp(unchecked.thickness..=checked.thickness, checked_progress),
    }
}

fn unchecked_geometry_for_rect(rect: Rect) -> HamburgerToggleButtonGeometry {
    let content_padding = content_padding_token();
    let content_width_px = (rect.width() - content_padding * 2.0).max(0.0);
    let content_height_px = (rect.height() - content_padding * 2.0).max(0.0);

    let bar_thickness_px = (content_width_px.min(content_height_px) * 0.08).clamp(1.0, f32::MAX);
    let bar_inset_px = bar_thickness_px.max(1.0);
    let bar_spacing_px = ((content_height_px - 2.0 * bar_inset_px) / 4.0)
        .min(content_height_px * 0.2)
        .max(0.0);
    let bar_full_width_px = (content_width_px - bar_inset_px * 2.0).max(0.0);

    let start_x = rect.left() + content_padding + bar_inset_px;
    let top_y = rect.top() + content_padding + content_height_px / 2.0 - bar_spacing_px;
    let mid_y = rect.top() + content_padding + content_height_px / 2.0;
    let bottom_y = rect.top() + content_padding + content_height_px / 2.0 + bar_spacing_px;

    HamburgerToggleButtonGeometry {
        top_start: Pos2::new(start_x, top_y),
        top_end: Pos2::new(start_x + bar_full_width_px, top_y),
        middle_start: Pos2::new(start_x, mid_y),
        middle_end: Pos2::new(start_x + bar_full_width_px, mid_y),
        bottom_start: Pos2::new(start_x, bottom_y),
        bottom_end: Pos2::new(start_x + bar_full_width_px, bottom_y),
        thickness: bar_thickness_px,
    }
}

fn checked_geometry_for_rect(rect: Rect) -> HamburgerToggleButtonGeometry {
    let base = unchecked_geometry_for_rect(rect);
    let bar_full_width_px = base.middle_end.x - base.middle_start.x;
    let bar_half_width_px = bar_full_width_px / 2.0;
    let rotation = CHECKED_ROTATION_DEGREES.to_radians();
    let top_delta = vec2(
        bar_half_width_px * rotation.cos(),
        -bar_half_width_px * rotation.sin(),
    );
    let bottom_delta = vec2(
        bar_half_width_px * rotation.cos(),
        bar_half_width_px * rotation.sin(),
    );
    let mid_y = base.middle_start.y;
    let start_x = base.middle_start.x;

    HamburgerToggleButtonGeometry {
        top_start: Pos2::new(start_x, mid_y),
        top_end: Pos2::new(start_x + top_delta.x, mid_y + top_delta.y),
        middle_start: Pos2::new(start_x, mid_y),
        middle_end: Pos2::new(start_x + bar_full_width_px, mid_y),
        bottom_start: Pos2::new(start_x, mid_y),
        bottom_end: Pos2::new(start_x + bottom_delta.x, mid_y + bottom_delta.y),
        thickness: base.thickness,
    }
}

fn lerp_pos2(from: Pos2, to: Pos2, t: f32) -> Pos2 {
    Pos2::new(egui::lerp(from.x..=to.x, t), egui::lerp(from.y..=to.y, t))
}
