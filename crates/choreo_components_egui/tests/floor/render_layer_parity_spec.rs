use crate::floor::floor_component::actions::FloorAction;
use crate::floor::floor_component::reducer::reduce;
use crate::floor::floor_component::state::FloorLayer;
use crate::floor::floor_component::state::FloorPosition;
use crate::floor::floor_component::state::FloorState;
use crate::floor::floor_component::state::Point;
use crate::floor::floor_component::state::TouchAction;
use crate::floor::floor_component::state::TouchDeviceType;

#[test]
fn draw_floor_builds_expected_layer_order_and_primitives() {
    let mut state = FloorState::default();

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

    assert_eq!(
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
    assert!(state.background_rect.is_some());
    assert!(state.header_overlay_rect.is_some());
    assert!(state.svg_overlay_bounds.is_some());
    assert!(!state.grid_lines.is_empty());
    assert!(!state.path_segments.is_empty());
    assert!(!state.dashed_path_segments.is_empty());
    assert_eq!(state.position_circles.len(), 3);
    assert_eq!(state.position_labels.len(), 3);
    assert_eq!(state.selection_segments.len(), 4);
    assert_eq!(state.center_mark_segments.len(), 2);
    assert_eq!(state.axis_labels.len(), 2);
    assert_eq!(state.legend_entries.len(), 1);
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
    assert!(state.floor_y >= state.header_height_px);
    assert_eq!(header.x, state.floor_x);
    assert_eq!(header.width, state.floor_width);
    assert!((header.y - (state.floor_y - state.header_height_px)).abs() < 0.001);
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
    assert_eq!(state.active_touches.len(), 1);

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

    assert!(state.active_touches.is_empty());
    assert!(state.pinch_distance.is_none());
    assert!(state.pointer_anchor.is_none());
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

    assert_eq!(state.last_touch_device, Some(TouchDeviceType::Pen));
}
