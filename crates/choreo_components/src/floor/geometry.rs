use egui::Rect;

use super::state::FloorState;
use super::state::Point;
use super::state::RectPrimitive;
use super::tokens::MIN_DANCER_RADIUS;

#[must_use]
pub(super) fn to_screen_point(canvas_rect: Rect, point: Point) -> egui::Pos2 {
    egui::pos2(
        canvas_rect.min.x + point.x as f32,
        canvas_rect.min.y + point.y as f32,
    )
}

#[must_use]
pub(super) fn to_screen_rect(canvas_rect: Rect, x: f64, y: f64, width: f64, height: f64) -> Rect {
    Rect::from_min_size(
        egui::pos2(canvas_rect.min.x + x as f32, canvas_rect.min.y + y as f32),
        egui::vec2(width as f32, height as f32),
    )
}

#[must_use]
pub(super) fn primitive_to_screen_rect(canvas_rect: Rect, primitive: RectPrimitive) -> Rect {
    to_screen_rect(
        canvas_rect,
        primitive.x,
        primitive.y,
        primitive.width,
        primitive.height,
    )
}

#[must_use]
pub(super) fn floor_position_radius(state: &FloorState) -> f32 {
    let width_meters = f64::from((state.floor_left + state.floor_right).max(1));
    let height_meters = f64::from((state.floor_front + state.floor_back).max(1));
    let scale_x = state.floor_width / width_meters;
    let scale_y = state.floor_height / height_meters;
    let scale = scale_x.min(scale_y);

    ((state.dancer_size.max(1.0) * scale) / 2.0) as f32
}

#[must_use]
pub(super) fn clamped_floor_position_radius(state: &FloorState) -> f32 {
    floor_position_radius(state).max(MIN_DANCER_RADIUS)
}
