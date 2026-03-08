use super::actions::FloorAction;
use super::state::AxisLabel;
use super::state::FloorLayer;
use super::state::FloorLayoutMetrics;
use super::state::FloorPosition;
use super::state::FloorState;
use super::state::LabeledPoint;
use super::state::LegendEntry;
use super::state::LineSegment;
use super::state::Point;
use super::state::PointerButton;
use super::state::RectPrimitive;
use super::state::RenderedFloorPosition;
use super::state::SceneRenderPosition;
use super::state::TouchAction;
use super::state::TouchDeviceType;

pub fn reduce(state: &mut FloorState, action: FloorAction) {
    match action {
        FloorAction::Initialize => {
            state.metrics = FloorLayoutMetrics::from_zoom(state.zoom);
            recompute_layout(state);
            recompute_geometry(state);
        }
        FloorAction::DrawFloor => {
            state.draw_count += 1;
            if !state.render_marked {
                state.render_marked = true;
                state.render_mark_count += 1;
            }
            recompute_layout(state);
            recompute_geometry(state);
        }
        FloorAction::RedrawFloor => {
            state.draw_count += 1;
            recompute_layout(state);
            recompute_geometry(state);
        }
        FloorAction::SetInteractionMode { mode } => {
            state.interaction_mode = mode;
        }
        FloorAction::SetPositions { positions } => {
            state.positions = positions;
            state.selected_positions.clear();
            recompute_geometry(state);
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
            recompute_geometry(state);
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
            recompute_geometry(state);
        }
        FloorAction::RotateSelectedAroundCenter { start, end } => {
            let Some(center) = selection_center(&state.positions, &state.selected_positions) else {
                return;
            };
            rotate_selected(state, center, start, end);
            recompute_geometry(state);
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
            recompute_geometry(state);
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
            recompute_geometry(state);
        }
        FloorAction::PlacePosition { point } => {
            state.positions.push(FloorPosition::new(point.x, point.y));
            recompute_geometry(state);
        }
        FloorAction::ClearSelection => {
            state.selected_positions.clear();
            state.selection_rectangle = None;
            recompute_geometry(state);
        }
        FloorAction::PointerPressed { point } => {
            state.pointer_anchor = Some(point);
        }
        FloorAction::PointerPressedWithContext {
            canvas_view,
            event_args,
        } => {
            state.last_canvas_view = Some(canvas_view);
            state.last_pointer_pressed = Some(event_args);
            if event_args.button != PointerButton::Primary {
                state.pointer_anchor = None;
                return;
            }
            reduce(
                state,
                FloorAction::PointerPressed {
                    point: event_args.position,
                },
            );
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
            recompute_layout(state);
            recompute_geometry(state);
        }
        FloorAction::PointerMovedWithContext {
            canvas_view,
            event_args,
        } => {
            state.last_canvas_view = Some(canvas_view);
            state.last_pointer_moved = Some(event_args);
            reduce(
                state,
                FloorAction::PointerMoved {
                    point: event_args.position,
                },
            );
        }
        FloorAction::PointerReleased { point } => {
            if let Some(last_tap) = state.last_tap_point
                && distance(last_tap, point) <= 3.0
            {
                state.transformation_matrix = super::state::Matrix::identity();
                recompute_layout(state);
                recompute_geometry(state);
            }
            state.last_tap_point = Some(point);
            state.pointer_anchor = None;
        }
        FloorAction::PointerReleasedWithContext {
            canvas_view,
            event_args,
        } => {
            state.last_canvas_view = Some(canvas_view);
            state.last_pointer_released = Some(event_args);
            reduce(
                state,
                FloorAction::PointerReleased {
                    point: event_args.position,
                },
            );
        }
        FloorAction::PointerWheelChanged {
            delta_x,
            delta_y,
            ctrl,
            cursor,
        } => {
            if should_pan_with_wheel(delta_x, delta_y, ctrl) {
                state.transformation_matrix.translate(delta_x, delta_y);
                recompute_layout(state);
                recompute_geometry(state);
                return;
            }

            let current = state.transformation_matrix.scale_x;
            let factor = if delta_y > 0.0 { 1.1 } else { 0.9 };
            let new_scale = (current * factor).clamp(0.1, 10.0);

            if let Some(cursor_point) = cursor {
                let world_x = (cursor_point.x - state.transformation_matrix.trans_x) / current;
                let world_y = (cursor_point.y - state.transformation_matrix.trans_y) / current;
                state.transformation_matrix.set_uniform_scale(new_scale);
                state.transformation_matrix.trans_x = cursor_point.x - world_x * new_scale;
                state.transformation_matrix.trans_y = cursor_point.y - world_y * new_scale;
            } else {
                state.transformation_matrix.set_uniform_scale(new_scale);
            }
            recompute_layout(state);
            recompute_geometry(state);
        }
        FloorAction::PointerWheelChangedWithContext {
            canvas_view,
            delta_x,
            delta_y,
            control_modifier,
            position,
        } => {
            state.last_canvas_view = Some(canvas_view);
            state.last_wheel_control_modifier = control_modifier;
            state.last_wheel_position = position;
            reduce(
                state,
                FloorAction::PointerWheelChanged {
                    delta_x,
                    delta_y,
                    ctrl: control_modifier,
                    cursor: position,
                },
            );
        }
        FloorAction::Touch {
            id,
            action,
            point,
            is_in_contact,
            device,
        } => {
            state.last_touch_device = Some(device);
            if device != TouchDeviceType::Touch {
                if action == TouchAction::Cancelled {
                    state.active_touches.clear();
                    state.pinch_distance = None;
                    state.pointer_anchor = None;
                }
                return;
            }
            match action {
                TouchAction::Pressed | TouchAction::Moved => {
                    if is_in_contact {
                        state.active_touches.insert(id, point);
                    }
                    if state.active_touches.len() == 2 {
                        let touch_points: Vec<Point> =
                            state.active_touches.values().copied().collect();
                        let pinch = distance(touch_points[0], touch_points[1]);
                        let previous = state.pinch_distance.replace(pinch);
                        if let Some(previous_distance) = previous
                            && previous_distance > 0.0001
                        {
                            let factor = pinch / previous_distance;
                            let current = state.transformation_matrix.scale_x;
                            state
                                .transformation_matrix
                                .set_uniform_scale(current * factor);
                            recompute_layout(state);
                            recompute_geometry(state);
                        }
                    }
                }
                TouchAction::Released => {
                    state.active_touches.remove(&id);
                    if state.active_touches.len() < 2 {
                        state.pinch_distance = None;
                    }
                }
                TouchAction::Cancelled => {
                    state.active_touches.clear();
                    state.pinch_distance = None;
                    state.pointer_anchor = None;
                }
            }
        }
        FloorAction::TouchWithContext {
            canvas_view,
            event_args,
        } => {
            state.last_canvas_view = Some(canvas_view);
            state.last_touch_event = Some(event_args);
            reduce(
                state,
                FloorAction::Touch {
                    id: event_args.id,
                    action: event_args.action,
                    point: event_args.location,
                    is_in_contact: event_args.in_contact,
                    device: event_args.device_type,
                },
            );
        }
        FloorAction::SetLayout {
            width_px,
            height_px,
        } => {
            state.layout_width_px = width_px.max(12.0);
            state.layout_height_px = height_px.max(12.0);
            recompute_layout(state);
            recompute_geometry(state);
        }
        FloorAction::SetAxisLabels { x_axis, y_axis } => {
            state.axis_labels = vec![
                AxisLabel {
                    text: x_axis,
                    position: Point::new(0.0, 0.0),
                },
                AxisLabel {
                    text: y_axis,
                    position: Point::new(0.0, 0.0),
                },
            ];
            recompute_geometry(state);
        }
        FloorAction::SetLegendEntries { entries } => {
            state.legend_entries = entries
                .into_iter()
                .map(|(label, color)| super::state::LegendEntry {
                    shortcut: String::new(),
                    name: label,
                    position_text: String::new(),
                    color,
                })
                .collect();
            recompute_geometry(state);
        }
        FloorAction::SetPlacementRemaining { count } => {
            state.placement_remaining = count;
        }
        FloorAction::SetSvgOverlay { svg_path } => {
            state.svg_path = svg_path;
            recompute_geometry(state);
        }
        FloorAction::ResetViewport => {
            state.transformation_matrix = super::state::Matrix::identity();
            recompute_layout(state);
            recompute_geometry(state);
        }
        FloorAction::SetZoom { zoom } => {
            state.zoom = zoom.max(0.1);
            state.metrics = FloorLayoutMetrics::from_zoom(state.zoom);
            recompute_layout(state);
            recompute_geometry(state);
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
            recompute_geometry(state);
        }
    }
}

pub fn refresh_render_geometry(state: &mut FloorState) {
    recompute_layout(state);
    recompute_geometry(state);
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

fn should_pan_with_wheel(delta_x: f64, delta_y: f64, control_modifier: bool) -> bool {
    if control_modifier {
        return false;
    }

    if delta_x.abs() > f64::EPSILON {
        return true;
    }

    !is_notched_wheel_delta(delta_y)
}

fn is_notched_wheel_delta(delta: f64) -> bool {
    const WHEEL_NOTCH_DELTA: f64 = 120.0;
    const WHEEL_NOTCH_EPSILON: f64 = 0.5;

    let magnitude = delta.abs();
    if magnitude <= f64::EPSILON {
        return false;
    }

    let notch_count = (magnitude / WHEEL_NOTCH_DELTA).round();
    if notch_count < 1.0 {
        return false;
    }

    let expected = notch_count * WHEEL_NOTCH_DELTA;
    (magnitude - expected).abs() <= WHEEL_NOTCH_EPSILON
}

fn recompute_layout(state: &mut FloorState) {
    let header_height = (60.0 * state.zoom).max(12.0);
    let content_height = (state.layout_height_px - header_height).max(12.0);
    let horizontal_meters = f64::from((state.floor_left + state.floor_right).max(1));
    let vertical_meters = f64::from((state.floor_front + state.floor_back).max(1));
    let padding = (46.0 * state.zoom).max(12.0);
    let scale_x =
        ((state.layout_width_px - (padding * 2.0)).max(12.0) / horizontal_meters).max(0.1);
    let scale_y = ((content_height - (padding * 2.0)).max(12.0) / vertical_meters).max(0.1);
    let meters_to_px = scale_x.min(scale_y);
    let base_floor_width = horizontal_meters * meters_to_px;
    let base_floor_height = vertical_meters * meters_to_px;
    let base_center_x = state.layout_width_px / 2.0;
    let base_center_y = header_height + (content_height / 2.0);
    let base_floor_x = base_center_x - f64::from(state.floor_left) * meters_to_px;
    let base_floor_y = base_center_y - f64::from(state.floor_front) * meters_to_px;
    let user_scale = state.transformation_matrix.scale_x.max(0.1);
    let user_translate_x = state.transformation_matrix.trans_x;
    let user_translate_y = state.transformation_matrix.trans_y;

    state.header_height_px = header_height;
    state.content_height_px = content_height;
    state.floor_x = base_floor_x * user_scale + user_translate_x;
    state.floor_y = base_floor_y * user_scale + user_translate_y;
    state.floor_width = base_floor_width * user_scale;
    state.floor_height = base_floor_height * user_scale;
    state.center_x = state.floor_x + (state.floor_width / 2.0);
    state.center_y = state.floor_y + (state.floor_height / 2.0);
}

fn recompute_geometry(state: &mut FloorState) {
    state.background_rect = Some(RectPrimitive::from_xywh(
        state.floor_x,
        state.floor_y,
        state.floor_width,
        state.floor_height,
    ));
    state.header_overlay_rect = Some(RectPrimitive::from_xywh(
        state.floor_x,
        state.floor_y - state.header_height_px,
        state.floor_width,
        state.header_height_px,
    ));
    state.legend_panel_rect = if !state.show_legend || !has_legend_entries(state) {
        None
    } else {
        let legend_margin = 48.0 * state.zoom;
        let legend_width = state.metrics.legend_panel_width;
        let legend_height = state.metrics.legend_panel_height;
        let legend_x = (state.floor_x + state.floor_width + legend_margin)
            .min((state.layout_width_px - legend_width).max(12.0))
            .max(12.0);
        let legend_y = state
            .floor_y
            .min((state.layout_height_px - legend_height).max(12.0))
            .max(12.0);
        Some(RectPrimitive::from_xywh(
            legend_x,
            legend_y,
            legend_width,
            legend_height,
        ))
    };
    state.svg_overlay_bounds = state.svg_path.as_ref().map(|_| {
        RectPrimitive::from_xywh(
            state.center_x - (state.floor_width / 2.0),
            state.center_y - (state.floor_height / 2.0),
            state.floor_width,
            state.floor_height,
        )
    });

    state.grid_lines.clear();
    if state.show_grid_lines {
        let grid_step = 1.0 / f64::from(state.grid_resolution.max(1));
        let max_horizontal = state.floor_left.max(state.floor_right);
        let max_vertical = state.floor_front.max(state.floor_back);
        let mut meter = grid_step;
        while meter <= f64::from(max_horizontal) + 0.0001 {
            if meter <= f64::from(state.floor_left) {
                let x = map_floor_coordinate_to_canvas(state, -meter, 0.0).x;
                state.grid_lines.push(LineSegment {
                    from: Point::new(x, state.floor_y),
                    to: Point::new(x, state.floor_y + state.floor_height),
                });
            }
            if meter <= f64::from(state.floor_right) {
                let x = map_floor_coordinate_to_canvas(state, meter, 0.0).x;
                state.grid_lines.push(LineSegment {
                    from: Point::new(x, state.floor_y),
                    to: Point::new(x, state.floor_y + state.floor_height),
                });
            }
            meter += grid_step;
        }

        meter = grid_step;
        while meter <= f64::from(max_vertical) + 0.0001 {
            if meter <= f64::from(state.floor_front) {
                let y = map_floor_coordinate_to_canvas(state, 0.0, meter).y;
                state.grid_lines.push(LineSegment {
                    from: Point::new(state.floor_x, y),
                    to: Point::new(state.floor_x + state.floor_width, y),
                });
            }
            if meter <= f64::from(state.floor_back) {
                let y = map_floor_coordinate_to_canvas(state, 0.0, -meter).y;
                state.grid_lines.push(LineSegment {
                    from: Point::new(state.floor_x, y),
                    to: Point::new(state.floor_x + state.floor_width, y),
                });
            }
            meter += grid_step;
        }
    }
    state.center_mark_segments = vec![
        LineSegment {
            from: Point::new(state.floor_x, state.center_y),
            to: Point::new(state.floor_x + state.floor_width, state.center_y),
        },
        LineSegment {
            from: Point::new(state.center_x, state.floor_y),
            to: Point::new(state.center_x, state.floor_y + state.floor_height),
        },
    ];

    state.path_segments.clear();
    state.dashed_path_segments.clear();
    state.rendered_positions.clear();
    if state.source_positions.is_empty() {
        let active_positions = if state.interpolated_positions.is_empty() {
            state.positions.clone()
        } else {
            state.interpolated_positions.clone()
        };

        for window in active_positions.windows(2) {
            let first = window[0];
            let second = window[1];
            state.path_segments.push(LineSegment {
                from: transform_position(state, first),
                to: transform_position(state, second),
            });
        }

        state.dashed_path_segments = build_dashed_segments(&state.path_segments, 24.0 * state.zoom);
        state.position_circles = active_positions
            .iter()
            .map(|position| transform_position(state, *position))
            .collect();
        state.position_labels = active_positions
            .iter()
            .enumerate()
            .map(|(index, position)| LabeledPoint {
                text: (index + 1).to_string(),
                point: transform_position(state, *position),
            })
            .collect();
    } else {
        state.position_circles.clear();
        state.position_labels.clear();
        state.rendered_positions = build_rendered_positions(state);
        state.position_circles = state
            .rendered_positions
            .iter()
            .map(|position| position.point)
            .collect();
        state.position_labels = state
            .rendered_positions
            .iter()
            .enumerate()
            .map(|(index, position)| {
                let label = if position.shortcut.trim().is_empty() {
                    (index + 1).to_string()
                } else {
                    position.shortcut.clone()
                };
                LabeledPoint {
                    text: label,
                    point: position.point,
                }
            })
            .collect();
        if state.draw_path_to {
            state.path_segments = build_scene_path_segments(
                &state.source_positions,
                &state.next_source_positions,
                false,
                state,
            );
        }
        if state.draw_path_from {
            state.dashed_path_segments = build_scene_path_segments(
                &state.previous_source_positions,
                &state.source_positions,
                true,
                state,
            );
        }
        if state.positions_at_side {
            state.axis_labels = build_side_axis_labels(state);
        } else {
            state.axis_labels.clear();
        }
        state.legend_entries = build_legend_entries(state);
    }

    state.selection_segments.clear();
    if let Some((start, end)) = state.selection_rectangle {
        let top_left = transform_point(state, Point::new(start.x.min(end.x), start.y.min(end.y)));
        let top_right = transform_point(state, Point::new(start.x.max(end.x), start.y.min(end.y)));
        let bottom_left =
            transform_point(state, Point::new(start.x.min(end.x), start.y.max(end.y)));
        let bottom_right =
            transform_point(state, Point::new(start.x.max(end.x), start.y.max(end.y)));
        state.selection_segments = vec![
            LineSegment {
                from: top_left,
                to: top_right,
            },
            LineSegment {
                from: top_right,
                to: bottom_right,
            },
            LineSegment {
                from: bottom_right,
                to: bottom_left,
            },
            LineSegment {
                from: bottom_left,
                to: top_left,
            },
        ];
    }

    if state.source_positions.is_empty() {
        refresh_axis_label_positions(state);
    }

    state.layer_order = vec![
        FloorLayer::Background,
        FloorLayer::GridLines,
        FloorLayer::FloorSvg,
        FloorLayer::PathSegments,
        FloorLayer::PositionCircles,
        FloorLayer::PositionNumbers,
        FloorLayer::SelectionSegments,
        FloorLayer::HeaderOverlay,
    ];
}

fn transform_position(state: &FloorState, position: FloorPosition) -> Point {
    transform_point(state, Point::new(position.x, position.y))
}

fn transform_point(state: &FloorState, point: Point) -> Point {
    Point::new(
        point.x * state.transformation_matrix.scale_x + state.transformation_matrix.trans_x,
        point.y * state.transformation_matrix.scale_y + state.transformation_matrix.trans_y,
    )
}

fn map_floor_coordinate_to_canvas(state: &FloorState, x: f64, y: f64) -> Point {
    let width_meters = f64::from((state.floor_left + state.floor_right).max(1));
    let height_meters = f64::from((state.floor_front + state.floor_back).max(1));
    let scale_x = state.floor_width / width_meters;
    let scale_y = state.floor_height / height_meters;
    let scale = scale_x.min(scale_y);
    Point::new(state.center_x + x * scale, state.center_y - y * scale)
}

fn build_rendered_positions(state: &FloorState) -> Vec<RenderedFloorPosition> {
    state
        .source_positions
        .iter()
        .enumerate()
        .map(|(index, position)| {
            let active =
                state
                    .interpolated_positions
                    .get(index)
                    .copied()
                    .unwrap_or(FloorPosition {
                        x: position.x,
                        y: position.y,
                    });
            RenderedFloorPosition {
                point: map_floor_coordinate_to_canvas(state, active.x, active.y),
                fill_color: apply_transparency(position.fill_color, state.transparency),
                border_color: apply_transparency(position.border_color, state.transparency),
                text_color: position.text_color,
                shortcut: position.shortcut.clone(),
                is_selected: state.selected_positions.contains(&index),
                has_dancer: position.has_dancer,
            }
        })
        .collect()
}

fn build_side_axis_labels(state: &FloorState) -> Vec<AxisLabel> {
    let mut x_values: Vec<f64> = state
        .source_positions
        .iter()
        .map(|position| position.x)
        .collect();
    let mut y_values: Vec<f64> = state
        .source_positions
        .iter()
        .map(|position| position.y)
        .collect();
    x_values.sort_by(|left, right| left.total_cmp(right));
    y_values.sort_by(|left, right| left.total_cmp(right));
    x_values.dedup_by(|left, right| (*left - *right).abs() < 0.001);
    y_values.dedup_by(|left, right| (*left - *right).abs() < 0.001);

    let mut labels = Vec::new();
    for value in x_values {
        let top_point = map_floor_coordinate_to_canvas(state, value, f64::from(state.floor_front));
        let bottom_point =
            map_floor_coordinate_to_canvas(state, value, -f64::from(state.floor_back));
        labels.push(AxisLabel {
            text: format_position_value(value),
            position: Point::new(
                top_point.x,
                state.floor_y - state.metrics.top_label_vertical_gap,
            ),
        });
        labels.push(AxisLabel {
            text: format_position_value(value),
            position: Point::new(
                bottom_point.x,
                state.floor_y + state.floor_height + state.metrics.bottom_label_vertical_gap,
            ),
        });
    }
    for value in y_values {
        let left_point = map_floor_coordinate_to_canvas(state, -f64::from(state.floor_left), value);
        let right_point =
            map_floor_coordinate_to_canvas(state, f64::from(state.floor_right), value);
        labels.push(AxisLabel {
            text: format_position_value(value),
            position: Point::new(
                state.floor_x - state.metrics.side_label_left_gap,
                left_point.y,
            ),
        });
        labels.push(AxisLabel {
            text: format_position_value(value),
            position: Point::new(
                state.floor_x + state.floor_width + state.metrics.side_label_right_gap,
                right_point.y,
            ),
        });
    }
    labels
}

fn build_legend_entries(state: &FloorState) -> Vec<LegendEntry> {
    let mut entries: Vec<LegendEntry> = state
        .source_positions
        .iter()
        .enumerate()
        .filter(|(_, position)| position.has_dancer)
        .map(|(index, position)| {
            let active =
                state
                    .interpolated_positions
                    .get(index)
                    .copied()
                    .unwrap_or(FloorPosition {
                        x: position.x,
                        y: position.y,
                    });
            LegendEntry {
                shortcut: position.shortcut.clone(),
                name: position.dancer_name.clone(),
                position_text: format_position_text(active.x, active.y),
                color: apply_transparency(position.fill_color, state.transparency),
            }
        })
        .collect();
    entries.sort_by(|left, right| {
        left.shortcut
            .to_ascii_lowercase()
            .cmp(&right.shortcut.to_ascii_lowercase())
            .then(
                left.name
                    .to_ascii_lowercase()
                    .cmp(&right.name.to_ascii_lowercase()),
            )
    });
    entries
}

fn has_legend_entries(state: &FloorState) -> bool {
    if state.source_positions.is_empty() {
        !state.legend_entries.is_empty()
    } else {
        state
            .source_positions
            .iter()
            .any(|position| position.has_dancer)
    }
}

fn build_scene_path_segments(
    from_positions: &[SceneRenderPosition],
    to_positions: &[SceneRenderPosition],
    use_darker_color: bool,
    state: &FloorState,
) -> Vec<LineSegment> {
    let mut segments = Vec::new();
    for to_position in to_positions {
        let Some(dancer_key) = to_position.dancer_key.as_deref() else {
            continue;
        };
        let Some(from_position) = from_positions
            .iter()
            .find(|candidate| candidate.dancer_key.as_deref() == Some(dancer_key))
        else {
            continue;
        };

        let curve_points = build_curve_points(from_position, to_position, 32);
        let mapped_points: Vec<Point> = curve_points
            .into_iter()
            .map(|point| map_floor_coordinate_to_canvas(state, point.x, point.y))
            .collect();
        if use_darker_color {
            segments.extend(build_dashed_segments_from_points(&mapped_points));
        } else {
            for window in mapped_points.windows(2) {
                segments.push(LineSegment {
                    from: window[0],
                    to: window[1],
                });
            }
        }
    }
    segments
}

fn build_curve_points(
    from_position: &SceneRenderPosition,
    to_position: &SceneRenderPosition,
    steps: usize,
) -> Vec<Point> {
    let start = Point::new(from_position.x, from_position.y);
    let end = Point::new(to_position.x, to_position.y);
    let Some(control1_x) = from_position.curve1_x else {
        return vec![start, end];
    };
    let Some(control1_y) = from_position.curve1_y else {
        return vec![start, end];
    };
    let control1 = Point::new(control1_x, control1_y);
    if let (Some(control2_x), Some(control2_y)) = (from_position.curve2_x, from_position.curve2_y) {
        return sample_cubic_curve(
            start,
            control1,
            Point::new(control2_x, control2_y),
            end,
            steps,
        );
    }
    sample_quadratic_curve(start, control1, end, steps)
}

fn sample_quadratic_curve(start: Point, control: Point, end: Point, steps: usize) -> Vec<Point> {
    (0..=steps)
        .map(|index| {
            let t = index as f64 / steps.max(1) as f64;
            let u = 1.0 - t;
            Point::new(
                u * u * start.x + 2.0 * u * t * control.x + t * t * end.x,
                u * u * start.y + 2.0 * u * t * control.y + t * t * end.y,
            )
        })
        .collect()
}

fn sample_cubic_curve(
    start: Point,
    control1: Point,
    control2: Point,
    end: Point,
    steps: usize,
) -> Vec<Point> {
    (0..=steps)
        .map(|index| {
            let t = index as f64 / steps.max(1) as f64;
            let u = 1.0 - t;
            let uu = u * u;
            let tt = t * t;
            Point::new(
                uu * u * start.x
                    + 3.0 * uu * t * control1.x
                    + 3.0 * u * tt * control2.x
                    + tt * t * end.x,
                uu * u * start.y
                    + 3.0 * uu * t * control1.y
                    + 3.0 * u * tt * control2.y
                    + tt * t * end.y,
            )
        })
        .collect()
}

fn build_dashed_segments_from_points(points: &[Point]) -> Vec<LineSegment> {
    let mut segments = Vec::new();
    for window in points.windows(2) {
        let partial = build_dashed_segments(
            &[LineSegment {
                from: window[0],
                to: window[1],
            }],
            12.0,
        );
        segments.extend(partial);
    }
    segments
}

fn apply_transparency(color: [u8; 4], transparency: f64) -> [u8; 4] {
    let opacity = (1.0 - transparency.clamp(0.0, 1.0)).clamp(0.0, 1.0);
    [
        color[0],
        color[1],
        color[2],
        (f64::from(color[3]) * opacity).round() as u8,
    ]
}

fn format_position_value(value: f64) -> String {
    let rounded = (value * 100.0).round() / 100.0;
    let mut text = format!("{rounded:.2}");
    while text.ends_with('0') {
        text.pop();
    }
    if text.ends_with('.') {
        text.pop();
    }
    if text.is_empty() {
        text.push('0');
    }
    text
}

fn format_position_text(x: f64, y: f64) -> String {
    format!("({x:.2}, {y:.2})")
}

fn refresh_axis_label_positions(state: &mut FloorState) {
    let axis_offset = 24.0 * state.zoom * state.transformation_matrix.scale_x.max(0.1);
    let x_text = state
        .axis_labels
        .first()
        .map(|label| label.text.clone())
        .unwrap_or_else(|| "X".to_string());
    let y_text = state
        .axis_labels
        .get(1)
        .map(|label| label.text.clone())
        .unwrap_or_else(|| "Y".to_string());

    state.axis_labels = vec![
        AxisLabel {
            text: x_text,
            position: Point::new(
                state.floor_x + (state.floor_width / 2.0),
                state.floor_y - axis_offset,
            ),
        },
        AxisLabel {
            text: y_text,
            position: Point::new(
                state.floor_x - axis_offset,
                state.floor_y + (state.floor_height / 2.0),
            ),
        },
    ];
}

fn build_dashed_segments(source: &[LineSegment], dash_length: f64) -> Vec<LineSegment> {
    let mut dashed = Vec::new();
    for segment in source {
        let dx = segment.to.x - segment.from.x;
        let dy = segment.to.y - segment.from.y;
        let length = (dx * dx + dy * dy).sqrt();
        if length <= 0.001 {
            continue;
        }
        let ux = dx / length;
        let uy = dy / length;
        let mut progress = 0.0;
        let step = dash_length.max(1.0);
        while progress < length {
            let dash_end = (progress + step).min(length);
            dashed.push(LineSegment {
                from: Point::new(
                    segment.from.x + ux * progress,
                    segment.from.y + uy * progress,
                ),
                to: Point::new(
                    segment.from.x + ux * dash_end,
                    segment.from.y + uy * dash_end,
                ),
            });
            progress += step * 2.0;
        }
    }
    dashed
}
