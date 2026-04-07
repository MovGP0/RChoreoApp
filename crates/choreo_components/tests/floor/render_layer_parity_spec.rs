use crate::floor::floor_component::actions::FloorAction;
use crate::floor::floor_component::reducer::reduce;
use crate::floor::floor_component::state::FloorLayer;
use crate::floor::floor_component::state::FloorPosition;
use crate::floor::floor_component::state::FloorState;
use crate::floor::floor_component::state::Point;
use crate::floor::floor_component::state::SceneRenderPosition;
use crate::floor::floor_component::state::TouchAction;
use crate::floor::floor_component::state::TouchDeviceType;

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
fn draw_floor_builds_expected_layer_order_and_primitives() {
    let mut state = FloorState {
        show_legend: true,
        ..FloorState::default()
    };

    reduce(
        &mut state,
        FloorAction::SetPositions {
            positions: vec![
                FloorPosition::new(120.0, 180.0),
                FloorPosition::new(180.0, 220.0),
                FloorPosition::new(260.0, 280.0),
            ],
        },
    );
    reduce(
        &mut state,
        FloorAction::SelectRectangle {
            start: Point::new(110.0, 170.0),
            end: Point::new(300.0, 300.0),
        },
    );
    reduce(
        &mut state,
        FloorAction::SetSvgOverlay {
            svg_path: Some("overlay.svg".to_string()),
        },
    );
    reduce(
        &mut state,
        FloorAction::SetAxisLabels {
            x_axis: "Left/Right".to_string(),
            y_axis: "Front/Back".to_string(),
        },
    );
    reduce(
        &mut state,
        FloorAction::SetLegendEntries {
            entries: vec![("Couple A".to_string(), [255, 0, 0, 255])],
        },
    );
    reduce(
        &mut state,
        FloorAction::SetPlacementRemaining { count: Some(2) },
    );
    reduce(&mut state, FloorAction::DrawFloor);

    let mut errors = Vec::new();

    check_eq!(
        errors,
        state.layer_order,
        vec![
            FloorLayer::Background,
            FloorLayer::GridLines,
            FloorLayer::FloorSvg,
            FloorLayer::PathSegments,
            FloorLayer::PositionCircles,
            FloorLayer::PositionNumbers,
            FloorLayer::SelectionSegments,
            FloorLayer::HeaderOverlay,
        ]
    );
    check!(errors, state.background_rect.is_some());
    check!(errors, state.header_overlay_rect.is_some());
    check!(errors, state.svg_overlay_bounds.is_some());
    check!(errors, !state.grid_lines.is_empty());
    check!(errors, !state.path_segments.is_empty());
    check!(errors, !state.dashed_path_segments.is_empty());
    check_eq!(errors, state.position_circles.len(), 3);
    check_eq!(errors, state.position_labels.len(), 3);
    check_eq!(errors, state.selection_segments.len(), 4);
    check_eq!(errors, state.center_mark_segments.len(), 2);
    check_eq!(errors, state.axis_labels.len(), 2);
    check_eq!(errors, state.legend_entries.len(), 1);
    check_eq!(errors, state.legend_entries[0].name, "Couple A");

    assert_no_errors(errors);
    assert_eq!(state.placement_remaining, Some(2));
}

#[test]
fn layout_reserves_header_and_binds_overlay_to_floor_coordinates() {
    let mut state = FloorState::default();

    reduce(
        &mut state,
        FloorAction::SetLayout {
            width_px: 1200.0,
            height_px: 900.0,
        },
    );

    let header = state
        .header_overlay_rect
        .expect("header overlay should be built from layout");
    let mut errors = Vec::new();

    check!(errors, state.floor_y >= state.header_height_px);
    check_eq!(errors, header.x, state.floor_x);
    check_eq!(errors, header.width, state.floor_width);
    check!(
        errors,
        (header.y - (state.floor_y - state.header_height_px)).abs() < 0.001
    );

    assert_no_errors(errors);
}

#[test]
fn legend_panel_uses_layout_metrics_and_sits_right_of_floor() {
    let mut state = FloorState {
        show_legend: true,
        ..FloorState::default()
    };

    reduce(
        &mut state,
        FloorAction::SetLayout {
            width_px: 2200.0,
            height_px: 900.0,
        },
    );
    reduce(
        &mut state,
        FloorAction::SetLegendEntries {
            entries: vec![("Couple A".to_string(), [255, 0, 0, 255])],
        },
    );
    reduce(&mut state, FloorAction::DrawFloor);

    let legend = state
        .legend_panel_rect
        .expect("legend panel rect should be built when entries are present");
    let mut errors = Vec::new();

    check!(errors, legend.x >= state.floor_x + state.floor_width);
    check!(
        errors,
        (legend.width - state.metrics.legend_panel_width).abs() < 0.001
    );
    check!(
        errors,
        (legend.height - state.metrics.legend_panel_height).abs() < 0.001
    );

    assert_no_errors(errors);
}

#[test]
fn draw_floor_projects_scene_render_positions_into_labels_legend_and_paths() {
    let mut state = FloorState {
        choreography_name: "Viennese Waltz".to_string(),
        scene_name: "Opening".to_string(),
        positions_at_side: true,
        show_legend: true,
        source_positions: vec![
            SceneRenderPosition {
                dancer_key: Some("id:1".to_string()),
                dancer_name: "Lead".to_string(),
                shortcut: "L".to_string(),
                x: -1.0,
                y: 1.0,
                curve1_x: None,
                curve1_y: None,
                curve2_x: None,
                curve2_y: None,
                fill_color: [220, 20, 60, 255],
                border_color: [128, 0, 0, 255],
                text_color: [255, 255, 255, 255],
                has_dancer: true,
            },
            SceneRenderPosition {
                dancer_key: Some("id:2".to_string()),
                dancer_name: "Follow".to_string(),
                shortcut: "F".to_string(),
                x: 1.0,
                y: -1.0,
                curve1_x: None,
                curve1_y: None,
                curve2_x: None,
                curve2_y: None,
                fill_color: [30, 144, 255, 255],
                border_color: [0, 64, 128, 255],
                text_color: [255, 255, 255, 255],
                has_dancer: true,
            },
        ],
        next_source_positions: vec![
            SceneRenderPosition {
                dancer_key: Some("id:1".to_string()),
                dancer_name: "Lead".to_string(),
                shortcut: "L".to_string(),
                x: 0.0,
                y: 2.0,
                curve1_x: Some(-0.5),
                curve1_y: Some(1.5),
                curve2_x: None,
                curve2_y: None,
                fill_color: [220, 20, 60, 255],
                border_color: [128, 0, 0, 255],
                text_color: [255, 255, 255, 255],
                has_dancer: true,
            },
            SceneRenderPosition {
                dancer_key: Some("id:2".to_string()),
                dancer_name: "Follow".to_string(),
                shortcut: "F".to_string(),
                x: 2.0,
                y: 0.0,
                curve1_x: None,
                curve1_y: None,
                curve2_x: None,
                curve2_y: None,
                fill_color: [30, 144, 255, 255],
                border_color: [0, 64, 128, 255],
                text_color: [255, 255, 255, 255],
                has_dancer: true,
            },
        ],
        ..FloorState::default()
    };

    reduce(
        &mut state,
        FloorAction::SetLayout {
            width_px: 1200.0,
            height_px: 900.0,
        },
    );
    reduce(&mut state, FloorAction::DrawFloor);

    let mut errors = Vec::new();

    check_eq!(errors, state.rendered_positions.len(), 2);
    check_eq!(errors, state.position_labels[0].text, "L");
    check!(errors, state.axis_labels.len() >= 4);
    check_eq!(errors, state.legend_entries[0].shortcut, "F");
    check_eq!(errors, state.legend_entries[0].position_text, "(1.00, -1.00)");
    check!(errors, !state.path_segments.is_empty());

    assert_no_errors(errors);
}

#[test]
fn touch_cancelled_clears_active_gesture_state() {
    let mut state = FloorState::default();

    reduce(
        &mut state,
        FloorAction::Touch {
            id: 7,
            action: TouchAction::Pressed,
            point: Point::new(20.0, 30.0),
            is_in_contact: true,
            device: TouchDeviceType::Touch,
        },
    );
    let mut errors = Vec::new();

    check_eq!(errors, state.active_touches.len(), 1);

    reduce(
        &mut state,
        FloorAction::Touch {
            id: 7,
            action: TouchAction::Cancelled,
            point: Point::new(20.0, 30.0),
            is_in_contact: false,
            device: TouchDeviceType::Touch,
        },
    );

    check!(errors, state.active_touches.is_empty());
    check!(errors, state.pinch_distance.is_none());
    check!(errors, state.pointer_anchor.is_none());

    assert_no_errors(errors);
}

#[test]
fn touch_device_variants_are_covered_for_contract_parity() {
    let mut state = FloorState::default();

    reduce(
        &mut state,
        FloorAction::Touch {
            id: 1,
            action: TouchAction::Pressed,
            point: Point::new(10.0, 10.0),
            is_in_contact: true,
            device: TouchDeviceType::Mouse,
        },
    );
    reduce(
        &mut state,
        FloorAction::Touch {
            id: 2,
            action: TouchAction::Pressed,
            point: Point::new(20.0, 20.0),
            is_in_contact: true,
            device: TouchDeviceType::Pen,
        },
    );

    let mut errors = Vec::new();

    check_eq!(errors, state.last_touch_device, Some(TouchDeviceType::Pen));
    check!(errors, state.active_touches.is_empty());
    check!(errors, state.pinch_distance.is_none());

    assert_no_errors(errors);
}
