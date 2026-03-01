use super::actions::FloorAction;
use super::state::FloorLayoutMetrics;
use super::state::FloorPosition;
use super::state::FloorState;
use super::state::Point;
use super::state::TouchAction;

pub fn reduce(state: &mut FloorState, action: FloorAction) {
    match action {
        FloorAction::Initialize => {
            state.metrics = FloorLayoutMetrics::from_zoom(state.zoom);
        }
        FloorAction::DrawFloor => {
            state.draw_count += 1;
            if !state.render_marked {
                state.render_marked = true;
                state.render_mark_count += 1;
            }
        }
        FloorAction::RedrawFloor => {
            state.draw_count += 1;
        }
        FloorAction::SetInteractionMode { mode } => {
            state.interaction_mode = mode;
        }
        FloorAction::SetPositions { positions } => {
            state.positions = positions;
            state.selected_positions.clear();
        }
        FloorAction::SelectRectangle { start, end } => {
            state.selection_rectangle = Some((start, end));
            state.selected_positions = state
                .positions
                .iter()
                .enumerate()
                .filter(|(_, position)| point_in_rectangle(position.x, position.y, start, end))
                .map(|(index, _)| index)
                .collect();
        }
        FloorAction::MoveSelectedByDelta { delta_x, delta_y } => {
            let selected = state.selected_positions.clone();
            for index in selected {
                if let Some(position) = state.positions.get_mut(index) {
                    position.x += delta_x;
                    position.y += delta_y;
                    if state.snap_to_grid {
                        position.x = snap_to_grid(position.x, state.grid_resolution);
                        position.y = snap_to_grid(position.y, state.grid_resolution);
                    }
                }
            }
        }
        FloorAction::RotateSelectedAroundCenter { start, end } => {
            let Some(center) = selection_center(&state.positions, &state.selected_positions) else {
                return;
            };
            rotate_selected(state, center, start, end);
        }
        FloorAction::SetPivotFromPoint { point } => {
            let nearest = state
                .positions
                .iter()
                .min_by(|left, right| {
                    distance_squared(point, Point::new(left.x, left.y))
                        .partial_cmp(&distance_squared(point, Point::new(right.x, right.y)))
                        .unwrap_or(std::cmp::Ordering::Equal)
                })
                .copied()
                .map(|position| Point::new(position.x, position.y));
            state.pivot = nearest;
        }
        FloorAction::RotateSelectedAroundPivot { start, end } => {
            let Some(pivot) = state.pivot else {
                return;
            };
            rotate_selected(state, pivot, start, end);
        }
        FloorAction::ScaleSelected { start, end } => {
            let Some(center) = selection_center(&state.positions, &state.selected_positions) else {
                return;
            };

            let from_distance = distance(center, start);
            if from_distance <= 0.0001 {
                return;
            }
            let to_distance = distance(center, end);
            let factor = to_distance / from_distance;

            let selected = state.selected_positions.clone();
            for index in selected {
                if let Some(position) = state.positions.get_mut(index) {
                    position.x = center.x + (position.x - center.x) * factor;
                    position.y = center.y + (position.y - center.y) * factor;
                }
            }
        }
        FloorAction::PlacePosition { point } => {
            state.positions.push(FloorPosition::new(point.x, point.y));
        }
        FloorAction::ClearSelection => {
            state.selected_positions.clear();
            state.selection_rectangle = None;
        }
        FloorAction::PointerPressed { point } => {
            state.pointer_anchor = Some(point);
        }
        FloorAction::PointerMoved { point } => {
            if state.active_touches.len() >= 2 {
                return;
            }
            let Some(anchor) = state.pointer_anchor else {
                return;
            };
            let delta_x = point.x - anchor.x;
            let delta_y = point.y - anchor.y;
            state.transformation_matrix.translate(delta_x, delta_y);
            state.pointer_anchor = Some(point);
        }
        FloorAction::PointerReleased { point } => {
            if let Some(last_tap) = state.last_tap_point
                && distance(last_tap, point) <= 3.0
            {
                state.transformation_matrix = super::state::Matrix::identity();
            }
            state.last_tap_point = Some(point);
            state.pointer_anchor = None;
        }
        FloorAction::PointerWheelChanged {
            delta_x,
            delta_y,
            ctrl: _,
        } => {
            if delta_x != 0.0 || !matches!(delta_y.abs(), value if (value - 120.0).abs() < 0.001) {
                state.transformation_matrix.translate(delta_x, delta_y);
                return;
            }

            let current = state.transformation_matrix.scale_x;
            let factor = if delta_y > 0.0 { 1.1 } else { 0.9 };
            state.transformation_matrix.set_uniform_scale(current * factor);
        }
        FloorAction::Touch {
            id,
            action,
            point,
            is_in_contact,
        } => match action {
            TouchAction::Pressed | TouchAction::Moved => {
                if is_in_contact {
                    state.active_touches.insert(id, point);
                }
                if state.active_touches.len() == 2 {
                    let touch_points: Vec<Point> = state.active_touches.values().copied().collect();
                    let pinch = distance(touch_points[0], touch_points[1]);
                    let previous = state.pinch_distance.replace(pinch);
                    if let Some(previous_distance) = previous
                        && previous_distance > 0.0001
                    {
                        let factor = pinch / previous_distance;
                        let current = state.transformation_matrix.scale_x;
                        state.transformation_matrix.set_uniform_scale(current * factor);
                    }
                }
            }
            TouchAction::Released => {
                state.active_touches.remove(&id);
                if state.active_touches.len() < 2 {
                    state.pinch_distance = None;
                }
            }
        },
        FloorAction::ResetViewport => {
            state.transformation_matrix = super::state::Matrix::identity();
        }
        FloorAction::SetZoom { zoom } => {
            state.zoom = zoom.max(0.1);
            state.metrics = FloorLayoutMetrics::from_zoom(state.zoom);
        }
        FloorAction::SetSnapToGrid {
            enabled,
            resolution,
        } => {
            state.snap_to_grid = enabled;
            state.grid_resolution = resolution.max(1);
        }
        FloorAction::InterpolateAudioPosition { from, to, progress } => {
            let clamped_progress = progress.clamp(0.0, 1.0);
            state.interpolated_positions = from
                .iter()
                .zip(to.iter())
                .map(|(from_position, to_position)| FloorPosition {
                    x: from_position.x + (to_position.x - from_position.x) * clamped_progress,
                    y: from_position.y + (to_position.y - from_position.y) * clamped_progress,
                })
                .collect();
        }
    }
}

fn point_in_rectangle(x: f64, y: f64, start: Point, end: Point) -> bool {
    let min_x = start.x.min(end.x);
    let max_x = start.x.max(end.x);
    let min_y = start.y.min(end.y);
    let max_y = start.y.max(end.y);
    x >= min_x && x <= max_x && y >= min_y && y <= max_y
}

fn selection_center(positions: &[FloorPosition], selected: &[usize]) -> Option<Point> {
    if selected.is_empty() {
        return None;
    }

    let mut sum_x = 0.0;
    let mut sum_y = 0.0;
    let mut count = 0.0;
    for index in selected {
        let position = positions.get(*index)?;
        sum_x += position.x;
        sum_y += position.y;
        count += 1.0;
    }

    Some(Point::new(sum_x / count, sum_y / count))
}

fn rotate_selected(state: &mut FloorState, center: Point, start: Point, end: Point) {
    let start_angle = (start.y - center.y).atan2(start.x - center.x);
    let end_angle = (end.y - center.y).atan2(end.x - center.x);
    let rotation = end_angle - start_angle;

    let selected = state.selected_positions.clone();
    for index in selected {
        if let Some(position) = state.positions.get_mut(index) {
            let translated_x = position.x - center.x;
            let translated_y = position.y - center.y;
            let rotated_x = translated_x * rotation.cos() - translated_y * rotation.sin();
            let rotated_y = translated_x * rotation.sin() + translated_y * rotation.cos();
            position.x = center.x + rotated_x;
            position.y = center.y + rotated_y;
        }
    }
}

fn snap_to_grid(value: f64, resolution: i32) -> f64 {
    let step = 1.0 / f64::from(resolution.max(1));
    (value / step).round() * step
}

fn distance(left: Point, right: Point) -> f64 {
    ((right.x - left.x).powi(2) + (right.y - left.y).powi(2)).sqrt()
}

fn distance_squared(left: Point, right: Point) -> f64 {
    (right.x - left.x).powi(2) + (right.y - left.y).powi(2)
}
