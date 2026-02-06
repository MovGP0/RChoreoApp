use std::cell::RefCell;
use std::collections::HashMap;
use std::path::Path;
use std::rc::Rc;

use choreo_master_mobile_json::Color as ChoreoColor;
use choreo_models::{PositionModel, SettingsPreferenceKeys};
use choreo_state_machine::{ApplicationStateMachine, StateKind};
use crossbeam_channel::Receiver;
use slint::{Color, Image, ModelRc, VecModel};

use crate::audio_player::AudioPlayerPositionChangedEvent;
use crate::floor::{FloorCanvasViewModel, Point, Rect, Size};
use crate::global::GlobalStateModel;
use crate::preferences::Preferences;
use crate::{AxisLabel, FloorCurve, FloorPosition, LegendEntry, LineSegment, ShellHost};

pub struct FloorAdapter {
    global_state: Rc<RefCell<GlobalStateModel>>,
    state_machine: Rc<RefCell<ApplicationStateMachine>>,
    preferences: Rc<dyn Preferences>,
    audio_position_receiver: Receiver<AudioPlayerPositionChangedEvent>,
    current_audio_seconds: Option<f64>,
    role_border_colors: HashMap<i32, Color>,
}

impl FloorAdapter {
    pub fn new(
        global_state: Rc<RefCell<GlobalStateModel>>,
        state_machine: Rc<RefCell<ApplicationStateMachine>>,
        preferences: Rc<dyn Preferences>,
        audio_position_receiver: Receiver<AudioPlayerPositionChangedEvent>,
    ) -> Self
    {
        Self {
            global_state,
            state_machine,
            preferences,
            audio_position_receiver,
            current_audio_seconds: None,
            role_border_colors: HashMap::new(),
        }
    }

    pub fn apply(&mut self, view: &ShellHost, view_model: &mut FloorCanvasViewModel) {
        self.drain_audio_position();

        self.apply_transform(view, view_model);
        self.update_bounds(view, view_model);

        let state_kind = self.state_machine.borrow().state().kind();
        if !is_floor_render_state(state_kind) {
            self.clear_floor(view);
            return;
        }

        let (
            choreography_name,
            scene_name,
            floor_front,
            floor_back,
            floor_left,
            floor_right,
            show_grid_lines,
            floor_color,
            dancer_size,
            transparency,
            positions_at_side,
            selected_positions,
            selection_rectangle,
            svg_file_path,
            dancer_count,
            is_place_mode,
            previous_scene,
            current_scene,
            next_scene,
        ) = {
            let global_state = self.global_state.borrow();
            let choreography = &global_state.choreography;
            let settings = &choreography.settings;
            let (previous_scene, current_scene, next_scene) = get_adjacent_scenes(&global_state);

            (
                choreography.name.clone(),
                global_state
                    .selected_scene
                    .as_ref()
                    .map(|scene| scene.name.clone())
                    .unwrap_or_default(),
                choreography.floor.size_front,
                choreography.floor.size_back,
                choreography.floor.size_left,
                choreography.floor.size_right,
                settings.grid_lines,
                color_from_choreo(&settings.floor_color),
                settings.dancer_size as f32,
                settings.transparency,
                settings.positions_at_side,
                global_state.selected_positions.clone(),
                global_state.selection_rectangle,
                global_state.svg_file_path.clone(),
                choreography.dancers.len(),
                global_state.is_place_mode,
                previous_scene,
                current_scene,
                next_scene,
            )
        };

        view.set_floor_choreography_name(choreography_name.as_str().into());
        view.set_floor_scene_name(scene_name.as_str().into());
        view.set_floor_front(floor_front);
        view.set_floor_back(floor_back);
        view.set_floor_left(floor_left);
        view.set_floor_right(floor_right);
        view.set_floor_show_grid_lines(show_grid_lines);
        view.set_floor_color(floor_color);
        view.set_floor_dancer_size(dancer_size);

        let floor_width_meters = (floor_left + floor_right) as f32;
        let floor_height_meters = (floor_front + floor_back) as f32;
        let render_transform =
            build_floor_render_transform(view_model, floor_width_meters, floor_height_meters);

        let current_scene = current_scene.as_ref();
        let scene_positions = current_scene.map(|scene| scene.positions.as_slice());

        let (positions, legend_entries) = self.build_positions_and_legend(
            scene_positions,
            current_scene,
            next_scene.as_ref(),
            &selected_positions,
            transparency,
        );

        view.set_floor_positions(ModelRc::new(VecModel::from(positions)));

        let (curves, dashed_segments) = self.build_curves(
            previous_scene.as_ref(),
            current_scene,
            next_scene.as_ref(),
            transparency,
            render_transform.as_ref(),
        );
        view.set_floor_curves(ModelRc::new(VecModel::from(curves)));
        view.set_floor_dashed_curve_segments(ModelRc::new(VecModel::from(dashed_segments)));

        let selection_segments = build_selection_segments(
            &selection_rectangle,
            state_kind,
            view.get_floor_subtitle_color(),
            render_transform.as_ref(),
        );
        view.set_floor_selection_segments(ModelRc::new(VecModel::from(selection_segments)));

        let (axis_labels_x, axis_labels_y, show_axis_labels) =
            build_axis_labels(scene_positions, positions_at_side);
        view.set_floor_axis_labels_x(ModelRc::new(VecModel::from(axis_labels_x)));
        view.set_floor_axis_labels_y(ModelRc::new(VecModel::from(axis_labels_y)));
        view.set_floor_show_axis_labels(show_axis_labels);

        let show_legend = self
            .preferences
            .get_bool(SettingsPreferenceKeys::SHOW_LEGEND, false)
            && !legend_entries.is_empty();
        view.set_floor_legend_entries(ModelRc::new(VecModel::from(legend_entries)));
        view.set_floor_show_legend(show_legend);

        let (svg_overlay, has_svg_overlay) = load_svg_overlay(&svg_file_path);
        view.set_floor_svg_overlay(svg_overlay);
        view.set_floor_has_svg_overlay(has_svg_overlay);

        let remaining_positions = remaining_positions(
            dancer_count,
            current_scene.map(|scene| scene.positions.len()).unwrap_or(0),
            state_kind,
            is_place_mode,
        );

        view.set_floor_remaining_positions(remaining_positions as i32);
        view.set_floor_show_placement_overlay(remaining_positions > 0);
    }

    pub fn poll_audio_position(&mut self) -> bool {
        self.drain_audio_position()
    }

    fn drain_audio_position(&mut self) -> bool {
        let mut updated = false;
        while let Ok(event) = self.audio_position_receiver.try_recv() {
            self.current_audio_seconds = Some(event.position_seconds);
            updated = true;
        }
        updated
    }

    fn update_bounds(&self, view: &ShellHost, view_model: &mut FloorCanvasViewModel) {
        let left = view.get_floor_bounds_left();
        let top = view.get_floor_bounds_top();
        let right = view.get_floor_bounds_right();
        let bottom = view.get_floor_bounds_bottom();
        view_model.set_floor_bounds(Rect::new(left, top, right, bottom));
        view_model.set_canvas_size(Size::new(
            view.get_floor_canvas_width(),
            view.get_floor_canvas_height(),
        ));
    }

    fn apply_transform(&self, view: &ShellHost, view_model: &FloorCanvasViewModel) {
        let matrix = view_model.transformation_matrix;
        view.set_floor_pan_x(matrix.trans_x());
        view.set_floor_pan_y(matrix.trans_y());
        view.set_floor_zoom_factor(matrix.scale_x());
    }

    fn clear_floor(&self, view: &ShellHost) {
        view.set_floor_positions(ModelRc::new(VecModel::from(Vec::<FloorPosition>::new())));
        view.set_floor_curves(ModelRc::new(VecModel::from(Vec::<FloorCurve>::new())));
        view.set_floor_dashed_curve_segments(ModelRc::new(VecModel::from(Vec::<LineSegment>::new())));
        view.set_floor_selection_segments(ModelRc::new(VecModel::from(Vec::<LineSegment>::new())));
        view.set_floor_axis_labels_x(ModelRc::new(VecModel::from(Vec::<AxisLabel>::new())));
        view.set_floor_axis_labels_y(ModelRc::new(VecModel::from(Vec::<AxisLabel>::new())));
        view.set_floor_legend_entries(ModelRc::new(VecModel::from(Vec::<LegendEntry>::new())));
        view.set_floor_show_axis_labels(false);
        view.set_floor_show_legend(false);
        view.set_floor_has_svg_overlay(false);
        view.set_floor_show_placement_overlay(false);
        view.set_floor_remaining_positions(0);
    }

    fn build_positions_and_legend(
        &mut self,
        positions: Option<&[PositionModel]>,
        current_scene: Option<&crate::scenes::SceneViewModel>,
        next_scene: Option<&crate::scenes::SceneViewModel>,
        selected_positions: &[PositionModel],
        transparency: f64,
    ) -> (Vec<FloorPosition>, Vec<LegendEntry>)
    {
        let Some(positions) = positions else {
            return (Vec::new(), Vec::new());
        };

        let interpolation = build_interpolation(current_scene, next_scene, self.current_audio_seconds);
        let mut floor_positions = Vec::with_capacity(positions.len());
        let mut legend_entries = Vec::new();

        for position in positions {
            let has_dancer = position.dancer.is_some();
            let mut draw_x = position.x;
            let mut draw_y = position.y;
            let mut shortcut = String::new();
            let mut fill_color = Color::from_rgb_u8(0, 0, 0);
            let mut border_color = Color::from_rgb_u8(0, 0, 0);
            let mut text_color = Color::from_rgb_u8(255, 255, 255);

            if let Some(dancer) = position.dancer.as_ref() {
                if let Some((t, next_positions)) = interpolation.as_ref()
                    && let Some(next_position) = get_next_position(next_positions, dancer)
                {
                    let (x, y) = interpolate_position(position, &next_position, *t);
                    draw_x = x;
                    draw_y = y;
                }

                fill_color = apply_transparency(color_from_choreo(&dancer.color), transparency);
                border_color = apply_transparency(self.role_border_color(&dancer.role), transparency);
                text_color = pick_black_or_white(fill_color);
                shortcut = dancer.shortcut.clone();

                legend_entries.push(LegendEntry {
                    color: fill_color,
                    shortcut: shortcut.as_str().into(),
                    name: dancer.name.as_str().into(),
                    position_text: format_position_text(draw_x, draw_y).into(),
                });
            }

            let is_selected = selected_positions.iter().any(|selected| selected == position);

            floor_positions.push(FloorPosition {
                x: draw_x as f32,
                y: draw_y as f32,
                fill_color,
                border_color,
                text_color,
                shortcut: shortcut.into(),
                is_selected,
                has_dancer,
            });
        }

        legend_entries.sort_by(|a, b| {
            let shortcut_cmp = a
                .shortcut
                .to_string()
                .to_lowercase()
                .cmp(&b.shortcut.to_string().to_lowercase());
            if shortcut_cmp != std::cmp::Ordering::Equal {
                return shortcut_cmp;
            }
            a.name
                .to_string()
                .to_lowercase()
                .cmp(&b.name.to_string().to_lowercase())
        });

        (floor_positions, legend_entries)
    }

    fn build_curves(
        &mut self,
        previous_scene: Option<&crate::scenes::SceneViewModel>,
        current_scene: Option<&crate::scenes::SceneViewModel>,
        next_scene: Option<&crate::scenes::SceneViewModel>,
        transparency: f64,
        render_transform: Option<&FloorRenderTransform>,
    ) -> (Vec<FloorCurve>, Vec<LineSegment>)
    {
        let Some(render_transform) = render_transform else {
            return (Vec::new(), Vec::new());
        };

        let mut curves = Vec::new();
        let mut dashed_segments = Vec::new();

        let draw_from = self
            .preferences
            .get_bool(SettingsPreferenceKeys::DRAW_PATH_FROM, true);
        let draw_to = self
            .preferences
            .get_bool(SettingsPreferenceKeys::DRAW_PATH_TO, true);

        if draw_from && let Some((from_scene, to_scene)) = previous_scene.zip(current_scene) {
            let segments = build_curves_between_scenes(
                from_scene,
                to_scene,
                true,
                transparency,
                render_transform,
            );
            dashed_segments.extend(segments);
        }

        if draw_to && let Some((from_scene, to_scene)) = current_scene.zip(next_scene) {
            let built = build_solid_curves_between_scenes(
                from_scene,
                to_scene,
                transparency,
                render_transform,
            );
            curves.extend(built);
        }

        (curves, dashed_segments)
    }

    fn role_border_color(&mut self, role: &choreo_models::RoleModel) -> Color {
        if let Some(cached) = self.role_border_colors.get(&role.z_index) {
            return *cached;
        }

        let color = color_from_choreo(&role.color);
        self.role_border_colors.insert(role.z_index, color);
        color
    }
}

struct FloorRenderTransform {
    scale: f32,
    offset_x: f32,
    offset_y: f32,
    width_px: f32,
    height_px: f32,
}

impl FloorRenderTransform {
    fn to_local(&self, point: Point) -> (f32, f32) {
        let center_x = self.offset_x + self.width_px / 2.0;
        let center_y = self.offset_y + self.height_px / 2.0;
        let x = center_x + point.x as f32 * self.scale;
        let y = center_y - point.y as f32 * self.scale;
        (x, y)
    }
}

fn build_floor_render_transform(
    view_model: &FloorCanvasViewModel,
    floor_width_meters: f32,
    floor_height_meters: f32,
) -> Option<FloorRenderTransform> {
    if !view_model.has_floor_bounds() {
        return None;
    }

    let bounds = view_model.floor_bounds();
    let width_px = bounds.width();
    let height_px = bounds.height();

    if width_px <= 0.0 || height_px <= 0.0 {
        return None;
    }

    if floor_width_meters <= 0.0 || floor_height_meters <= 0.0 {
        return None;
    }

    let scale_x = width_px / floor_width_meters;
    let scale_y = height_px / floor_height_meters;
    let scale = scale_x.min(scale_y);

    Some(FloorRenderTransform {
        scale,
        offset_x: bounds.left,
        offset_y: bounds.top,
        width_px,
        height_px,
    })
}

fn is_floor_render_state(state_kind: StateKind) -> bool {
    StateKind::ViewSceneState.is_assignable_from(state_kind)
        || StateKind::PlacePositionsState.is_assignable_from(state_kind)
        || StateKind::MovePositionsState.is_assignable_from(state_kind)
        || StateKind::RotateAroundCenterState.is_assignable_from(state_kind)
        || StateKind::ScalePositionsState.is_assignable_from(state_kind)
        || StateKind::ScaleAroundDancerState.is_assignable_from(state_kind)
}

fn build_axis_labels(
    positions: Option<&[PositionModel]>,
    positions_at_side: bool,
) -> (Vec<AxisLabel>, Vec<AxisLabel>, bool) {
    if !positions_at_side {
        return (Vec::new(), Vec::new(), false);
    }

    let Some(positions) = positions else {
        return (Vec::new(), Vec::new(), false);
    };

    if positions.is_empty() {
        return (Vec::new(), Vec::new(), false);
    }

    let mut x_values = Vec::new();
    let mut y_values = Vec::new();

    for position in positions {
        x_values.push(position.x);
        y_values.push(position.y);
    }

    x_values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    x_values.dedup_by(|a, b| (*a - *b).abs() < f64::EPSILON);

    y_values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    y_values.dedup_by(|a, b| (*a - *b).abs() < f64::EPSILON);

    let axis_labels_x = x_values
        .iter()
        .map(|value| AxisLabel {
            value: *value as f32,
            text: format_position_value(*value).into(),
        })
        .collect::<Vec<_>>();

    let axis_labels_y = y_values
        .iter()
        .map(|value| AxisLabel {
            value: *value as f32,
            text: format_position_value(*value).into(),
        })
        .collect::<Vec<_>>();

    (axis_labels_x, axis_labels_y, true)
}

fn build_selection_segments(
    selection_rectangle: &Option<crate::global::SelectionRectangle>,
    state_kind: StateKind,
    color: Color,
    render_transform: Option<&FloorRenderTransform>,
) -> Vec<LineSegment> {
    if state_kind != StateKind::MovePositionsSelectionState {
        return Vec::new();
    }

    let Some(rectangle) = selection_rectangle else {
        return Vec::new();
    };

    let start = rectangle.start;
    let end = rectangle.end;
    let left = start.x.min(end.x);
    let right = start.x.max(end.x);
    let top = start.y.min(end.y);
    let bottom = start.y.max(end.y);

    let points = [
        Point::new(left, top),
        Point::new(right, top),
        Point::new(right, bottom),
        Point::new(left, bottom),
        Point::new(left, top),
    ];

    let Some(render_transform) = render_transform else {
        return Vec::new();
    };

    build_dashed_segments_from_points(&points, 6.0, 6.0, color, 2.0, render_transform)
}

fn build_curves_between_scenes(
    from_scene: &crate::scenes::SceneViewModel,
    to_scene: &crate::scenes::SceneViewModel,
    use_darker_color: bool,
    transparency: f64,
    render_transform: &FloorRenderTransform,
) -> Vec<LineSegment> {
    let from_positions = build_positions_by_dancer_key(from_scene);
    if from_positions.is_empty() {
        return Vec::new();
    }

    let mut segments = Vec::new();

    for to_position in &to_scene.positions {
        let Some(dancer) = to_position.dancer.as_ref() else {
            continue;
        };

        let Some(from_position) = get_position_by_dancer_key(&from_positions, dancer) else {
            continue;
        };

        let mut color = color_from_choreo(&dancer.color);
        if use_darker_color {
            color = darken_color(color, 0.7);
        }
        color = apply_transparency(color, transparency);

        let curve_segments = build_dashed_curve_segments(
            from_position,
            to_position,
            color,
            2.0,
            render_transform,
        );
        segments.extend(curve_segments);
    }

    segments
}

fn build_solid_curves_between_scenes(
    from_scene: &crate::scenes::SceneViewModel,
    to_scene: &crate::scenes::SceneViewModel,
    transparency: f64,
    render_transform: &FloorRenderTransform,
) -> Vec<FloorCurve> {
    let from_positions = build_positions_by_dancer_key(from_scene);
    if from_positions.is_empty() {
        return Vec::new();
    }

    let mut curves = Vec::new();

    for to_position in &to_scene.positions {
        let Some(dancer) = to_position.dancer.as_ref() else {
            continue;
        };

        let Some(from_position) = get_position_by_dancer_key(&from_positions, dancer) else {
            continue;
        };

        let color = apply_transparency(color_from_choreo(&dancer.color), transparency);
        curves.push(build_curve(
            from_position,
            to_position,
            color,
            render_transform,
        ));
    }

    curves
}

fn build_curve(
    from_position: &PositionModel,
    to_position: &PositionModel,
    color: Color,
    render_transform: &FloorRenderTransform,
) -> FloorCurve {
    let mut curve = FloorCurve {
        start_x: from_position.x as f32,
        start_y: from_position.y as f32,
        end_x: to_position.x as f32,
        end_y: to_position.y as f32,
        control1_x: 0.0,
        control1_y: 0.0,
        control2_x: 0.0,
        control2_y: 0.0,
        has_control1: false,
        has_control2: false,
        color,
        commands: "".into(),
    };

    if let (Some(c1x), Some(c1y)) = (from_position.curve1_x, from_position.curve1_y) {
        curve.control1_x = c1x as f32;
        curve.control1_y = c1y as f32;
        curve.has_control1 = true;
    }

    if let (Some(c2x), Some(c2y)) = (from_position.curve2_x, from_position.curve2_y) {
        curve.control2_x = c2x as f32;
        curve.control2_y = c2y as f32;
        curve.has_control2 = true;
    }

    let control1 = if curve.has_control1 {
        Some(Point::new(curve.control1_x as f64, curve.control1_y as f64))
    } else {
        None
    };
    let control2 = if curve.has_control2 {
        Some(Point::new(curve.control2_x as f64, curve.control2_y as f64))
    } else {
        None
    };

    curve.commands = build_curve_commands(
        render_transform,
        Point::new(from_position.x, from_position.y),
        Point::new(to_position.x, to_position.y),
        control1,
        control2,
    )
    .into();

    curve
}

fn build_dashed_curve_segments(
    from_position: &PositionModel,
    to_position: &PositionModel,
    color: Color,
    thickness: f32,
    render_transform: &FloorRenderTransform,
) -> Vec<LineSegment> {
    let points = build_curve_points(from_position, to_position, 64);
    build_dashed_segments_from_points(
        &points,
        6.0,
        6.0,
        color,
        thickness,
        render_transform,
    )
}

fn build_curve_points(
    from_position: &PositionModel,
    to_position: &PositionModel,
    steps: usize,
) -> Vec<Point> {
    let start = Point::new(from_position.x, from_position.y);
    let end = Point::new(to_position.x, to_position.y);

    let Some(c1x) = from_position.curve1_x else {
        return vec![start, end];
    };
    let Some(c1y) = from_position.curve1_y else {
        return vec![start, end];
    };

    let control1 = Point::new(c1x, c1y);

    if let (Some(c2x), Some(c2y)) = (from_position.curve2_x, from_position.curve2_y) {
        let control2 = Point::new(c2x, c2y);
        return sample_cubic_curve(start, control1, control2, end, steps);
    }

    sample_quadratic_curve(start, control1, end, steps)
}

fn sample_quadratic_curve(start: Point, control: Point, end: Point, steps: usize) -> Vec<Point> {
    let mut points = Vec::with_capacity(steps + 1);
    for step in 0..=steps {
        let t = step as f64 / steps as f64;
        let u = 1.0 - t;
        let x = u * u * start.x + 2.0 * u * t * control.x + t * t * end.x;
        let y = u * u * start.y + 2.0 * u * t * control.y + t * t * end.y;
        points.push(Point::new(x, y));
    }
    points
}

fn sample_cubic_curve(
    start: Point,
    control1: Point,
    control2: Point,
    end: Point,
    steps: usize,
) -> Vec<Point> {
    let mut points = Vec::with_capacity(steps + 1);
    for step in 0..=steps {
        let t = step as f64 / steps as f64;
        let u = 1.0 - t;
        let tt = t * t;
        let uu = u * u;
        let uuu = uu * u;
        let ttt = tt * t;

        let x = uuu * start.x
            + 3.0 * uu * t * control1.x
            + 3.0 * u * tt * control2.x
            + ttt * end.x;
        let y = uuu * start.y
            + 3.0 * uu * t * control1.y
            + 3.0 * u * tt * control2.y
            + ttt * end.y;
        points.push(Point::new(x, y));
    }
    points
}

fn build_dashed_segments_from_points(
    points: &[Point],
    dash: f64,
    gap: f64,
    color: Color,
    thickness: f32,
    render_transform: &FloorRenderTransform,
) -> Vec<LineSegment> {
    if points.len() < 2 {
        return Vec::new();
    }

    let mut segments = Vec::new();
    let mut draw = true;
    let mut remaining = dash;

    let mut current = points[0];
    for next in points.iter().skip(1) {
        let mut segment_start = current;
        let mut dx = next.x - segment_start.x;
        let mut dy = next.y - segment_start.y;
        let mut segment_length = (dx * dx + dy * dy).sqrt();
        if segment_length <= f64::EPSILON {
            current = *next;
            continue;
        }

        while segment_length > f64::EPSILON {
            let step = remaining.min(segment_length);
            let t = step / segment_length;
            let end_point = Point::new(
                segment_start.x + dx * t,
                segment_start.y + dy * t,
            );

            if draw {
                let commands =
                    build_line_commands(render_transform, segment_start, end_point).into();
                segments.push(LineSegment {
                    start_x: segment_start.x as f32,
                    start_y: segment_start.y as f32,
                    end_x: end_point.x as f32,
                    end_y: end_point.y as f32,
                    color,
                    thickness,
                    commands,
                });
            }

            segment_start = end_point;
            segment_length -= step;
            if remaining <= step + f64::EPSILON {
                draw = !draw;
                remaining = if draw { dash } else { gap };
            } else {
                remaining -= step;
            }

            if segment_length > f64::EPSILON {
                dx = next.x - segment_start.x;
                dy = next.y - segment_start.y;
                segment_length = (dx * dx + dy * dy).sqrt();
            }
        }

        current = *next;
    }

    segments
}

fn build_line_commands(
    render_transform: &FloorRenderTransform,
    start: Point,
    end: Point,
) -> String {
    let (start_x, start_y) = render_transform.to_local(start);
    let (end_x, end_y) = render_transform.to_local(end);

    format!(
        "M {:.3} {:.3} L {:.3} {:.3}",
        start_x, start_y, end_x, end_y
    )
}

fn build_curve_commands(
    render_transform: &FloorRenderTransform,
    start: Point,
    end: Point,
    control1: Option<Point>,
    control2: Option<Point>,
) -> String {
    let (start_x, start_y) = render_transform.to_local(start);
    let (end_x, end_y) = render_transform.to_local(end);

    match (control1, control2) {
        (Some(control1), Some(control2)) => {
            let (c1x, c1y) = render_transform.to_local(control1);
            let (c2x, c2y) = render_transform.to_local(control2);
            format!(
                "M {:.3} {:.3} C {:.3} {:.3} {:.3} {:.3} {:.3} {:.3}",
                start_x, start_y, c1x, c1y, c2x, c2y, end_x, end_y
            )
        }
        (Some(control1), None) => {
            let (c1x, c1y) = render_transform.to_local(control1);
            format!(
                "M {:.3} {:.3} Q {:.3} {:.3} {:.3} {:.3}",
                start_x, start_y, c1x, c1y, end_x, end_y
            )
        }
        _ => format!(
            "M {:.3} {:.3} L {:.3} {:.3}",
            start_x, start_y, end_x, end_y
        ),
    }
}

fn remaining_positions(
    dancer_count: usize,
    position_count: usize,
    state_kind: StateKind,
    is_place_mode: bool,
) -> usize {
    if !StateKind::PlacePositionsState.is_assignable_from(state_kind) {
        return 0;
    }

    if !is_place_mode {
        return 0;
    }

    dancer_count.saturating_sub(position_count)
}

fn build_positions_by_dancer_key(
    scene: &crate::scenes::SceneViewModel,
) -> HashMap<String, PositionModel> {
    let mut lookup = HashMap::new();
    for position in &scene.positions {
        let Some(dancer) = position.dancer.as_ref() else {
            continue;
        };

        if let Some(key) = dancer_key(dancer) {
            lookup.insert(key, position.clone());
        }
    }
    lookup
}

fn get_position_by_dancer_key<'a>(
    lookup: &'a HashMap<String, PositionModel>,
    dancer: &choreo_models::DancerModel,
) -> Option<&'a PositionModel> {
    let key = dancer_key(dancer)?;
    lookup.get(&key)
}

fn get_next_position(
    lookup: &HashMap<String, PositionModel>,
    dancer: &choreo_models::DancerModel,
) -> Option<PositionModel> {
    let key = dancer_key(dancer)?;
    lookup.get(&key).cloned()
}

fn dancer_key(dancer: &choreo_models::DancerModel) -> Option<String> {
    if dancer.dancer_id.0 > 0 {
        return Some(format!("id:{}", dancer.dancer_id.0));
    }

    if !dancer.shortcut.trim().is_empty() {
        return Some(format!("shortcut:{}", dancer.shortcut));
    }

    if !dancer.name.trim().is_empty() {
        return Some(format!("name:{}", dancer.name));
    }

    None
}

fn build_interpolation(
    current_scene: Option<&crate::scenes::SceneViewModel>,
    next_scene: Option<&crate::scenes::SceneViewModel>,
    current_audio_seconds: Option<f64>,
) -> Option<(f64, HashMap<String, PositionModel>)> {
    let current_audio_seconds = current_audio_seconds?;
    let current_scene = current_scene?;
    let next_scene = next_scene?;

    let current_timestamp = current_scene.timestamp?;
    let next_timestamp = next_scene.timestamp?;

    let duration = next_timestamp - current_timestamp;
    if duration <= 0.0 {
        return None;
    }

    let raw_t = (current_audio_seconds - current_timestamp) / duration;
    if !(0.0..=1.0).contains(&raw_t) {
        return None;
    }

    let next_positions = build_positions_by_dancer_key(next_scene);
    Some((raw_t, next_positions))
}

fn interpolate_position(
    from_position: &PositionModel,
    to_position: &PositionModel,
    t: f64,
) -> (f64, f64) {
    let Some(curve1_x) = from_position.curve1_x else {
        return (
            lerp(from_position.x, to_position.x, t),
            lerp(from_position.y, to_position.y, t),
        );
    };

    let Some(curve1_y) = from_position.curve1_y else {
        return (
            lerp(from_position.x, to_position.x, t),
            lerp(from_position.y, to_position.y, t),
        );
    };

    if let (Some(curve2_x), Some(curve2_y)) = (from_position.curve2_x, from_position.curve2_y) {
        return (
            cubic_bezier(from_position.x, curve1_x, curve2_x, to_position.x, t),
            cubic_bezier(from_position.y, curve1_y, curve2_y, to_position.y, t),
        );
    }

    (
        quadratic_bezier(from_position.x, curve1_x, to_position.x, t),
        quadratic_bezier(from_position.y, curve1_y, to_position.y, t),
    )
}

fn lerp(start: f64, end: f64, t: f64) -> f64 {
    start + (end - start) * t
}

fn quadratic_bezier(p0: f64, p1: f64, p2: f64, t: f64) -> f64 {
    let u = 1.0 - t;
    u * u * p0 + 2.0 * u * t * p1 + t * t * p2
}

fn cubic_bezier(p0: f64, p1: f64, p2: f64, p3: f64, t: f64) -> f64 {
    let u = 1.0 - t;
    let uu = u * u;
    let tt = t * t;
    uu * u * p0 + 3.0 * uu * t * p1 + 3.0 * u * tt * p2 + tt * t * p3
}

fn get_adjacent_scenes(
    global_state: &GlobalStateModel,
) -> (
    Option<crate::scenes::SceneViewModel>,
    Option<crate::scenes::SceneViewModel>,
    Option<crate::scenes::SceneViewModel>,
) {
    let selected_scene = global_state.selected_scene.clone();
    let Some(selected_scene) = selected_scene else {
        return (None, None, None);
    };

    let scenes = &global_state.scenes;
    let index = scenes
        .iter()
        .position(|scene| scene.scene_id == selected_scene.scene_id)
        .or_else(|| scenes.iter().position(|scene| scene.name == selected_scene.name));

    let Some(index) = index else {
        return (None, None, None);
    };

    let previous = if index > 0 { scenes.get(index - 1).cloned() } else { None };
    let current = scenes.get(index).cloned();
    let next = scenes.get(index + 1).cloned();

    (previous, current, next)
}

fn color_from_choreo(color: &ChoreoColor) -> Color {
    Color::from_argb_u8(color.a, color.r, color.g, color.b)
}

fn apply_transparency(color: Color, transparency: f64) -> Color {
    let clamped = transparency.clamp(0.0, 1.0);
    let opacity = 1.0 - clamped;
    let alpha = (opacity * 255.0).round().clamp(0.0, 255.0) as u8;
    Color::from_argb_u8(alpha, color.red(), color.green(), color.blue())
}

fn darken_color(color: Color, lightness_scale: f32) -> Color {
    let (r, g, b, a) = (
        color.red() as f32,
        color.green() as f32,
        color.blue() as f32,
        color.alpha() as f32,
    );
    let (h, s, l) = rgb_to_hsl(r, g, b);
    let new_l = (l * lightness_scale).clamp(0.0, 100.0);
    let (nr, ng, nb) = hsl_to_rgb(h, s, new_l);
    Color::from_argb_u8(a as u8, nr as u8, ng as u8, nb as u8)
}

fn rgb_to_hsl(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let r = r / 255.0;
    let g = g / 255.0;
    let b = b / 255.0;

    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let delta = max - min;

    let l = (max + min) / 2.0;
    if delta == 0.0 {
        return (0.0, 0.0, l * 100.0);
    }

    let s = if l < 0.5 { delta / (max + min) } else { delta / (2.0 - max - min) };

    let mut h = if max == r {
        (g - b) / delta + if g < b { 6.0 } else { 0.0 }
    } else if max == g {
        (b - r) / delta + 2.0
    } else {
        (r - g) / delta + 4.0
    };

    h *= 60.0;
    (h, s * 100.0, l * 100.0)
}

fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (f32, f32, f32) {
    let s = s / 100.0;
    let l = l / 100.0;

    if s == 0.0 {
        let val = (l * 255.0).round();
        return (val, val, val);
    }

    let q = if l < 0.5 { l * (1.0 + s) } else { l + s - l * s };
    let p = 2.0 * l - q;

    let hk = h / 360.0;
    let t_r = hk + 1.0 / 3.0;
    let t_g = hk;
    let t_b = hk - 1.0 / 3.0;

    let r = hue_to_rgb(p, q, t_r);
    let g = hue_to_rgb(p, q, t_g);
    let b = hue_to_rgb(p, q, t_b);

    ((r * 255.0).round(), (g * 255.0).round(), (b * 255.0).round())
}

fn hue_to_rgb(p: f32, q: f32, mut t: f32) -> f32 {
    if t < 0.0 {
        t += 1.0;
    }
    if t > 1.0 {
        t -= 1.0;
    }
    if t < 1.0 / 6.0 {
        return p + (q - p) * 6.0 * t;
    }
    if t < 1.0 / 2.0 {
        return q;
    }
    if t < 2.0 / 3.0 {
        return p + (q - p) * (2.0 / 3.0 - t) * 6.0;
    }
    p
}

fn pick_black_or_white(color: Color) -> Color {
    let luminance = relative_luminance(color.red(), color.green(), color.blue());
    let contrast_black = (luminance + 0.05) / 0.05;
    let contrast_white = 1.05 / (luminance + 0.05);
    if contrast_white > contrast_black {
        Color::from_rgb_u8(255, 255, 255)
    } else {
        Color::from_rgb_u8(0, 0, 0)
    }
}

fn relative_luminance(r: u8, g: u8, b: u8) -> f32 {
    let r = linearize_channel(r as f32 / 255.0);
    let g = linearize_channel(g as f32 / 255.0);
    let b = linearize_channel(b as f32 / 255.0);
    0.2126 * r + 0.7152 * g + 0.0722 * b
}

fn linearize_channel(srgb: f32) -> f32 {
    if srgb <= 0.04045 {
        srgb / 12.92
    } else {
        ((srgb + 0.055) / 1.055).powf(2.4)
    }
}

fn format_position_value(value: f64) -> String {
    let mut text = format!("{value:.2}");
    if let Some(dot) = text.find('.') {
        while text.ends_with('0') {
            text.pop();
        }
        if text.ends_with('.') {
            text.pop();
        }
        if text.len() == dot {
            text.push('0');
        }
    }
    text
}

fn format_position_text(x: f64, y: f64) -> String {
    format!("({}, {})", format_position_value(x), format_position_value(y))
}

fn load_svg_overlay(path: &Option<String>) -> (Image, bool) {
    let Some(path) = path.as_ref() else {
        return (Image::default(), false);
    };

    match Image::load_from_path(Path::new(path)) {
        Ok(image) => (image, true),
        Err(_) => (Image::default(), false),
    }
}
