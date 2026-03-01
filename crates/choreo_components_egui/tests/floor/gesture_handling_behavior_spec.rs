use crate::floor::floor_component::actions::FloorAction;
use crate::floor::floor_component::reducer::reduce;
use crate::floor::floor_component::state::FloorState;
use crate::floor::floor_component::state::Point;
use crate::floor::floor_component::state::TouchAction;

#[test]
fn gesture_handling_applies_pan_zoom_and_reset_semantics() {
    let mut state = FloorState::default();

    reduce(
        &mut state,
        FloorAction::PointerPressed {
            point: Point::new(10.0, 10.0),
        },
    );
    reduce(
        &mut state,
        FloorAction::PointerMoved {
            point: Point::new(30.0, 25.0),
        },
    );
    assert!((state.transformation_matrix.trans_x - 20.0).abs() < 0.0001);
    assert!((state.transformation_matrix.trans_y - 15.0).abs() < 0.0001);

    reduce(
        &mut state,
        FloorAction::PointerWheelChanged {
            delta_x: 0.0,
            delta_y: 120.0,
            ctrl: false,
        },
    );
    assert!(state.transformation_matrix.scale_x > 1.0);

    reduce(
        &mut state,
        FloorAction::PointerWheelChanged {
            delta_x: 14.0,
            delta_y: -10.0,
            ctrl: false,
        },
    );
    assert!((state.transformation_matrix.trans_x - 34.0).abs() < 0.0001);
    assert!((state.transformation_matrix.trans_y - 5.0).abs() < 0.0001);

    reduce(
        &mut state,
        FloorAction::Touch {
            id: 1,
            action: TouchAction::Pressed,
            point: Point::new(40.0, 50.0),
            is_in_contact: true,
        },
    );
    reduce(
        &mut state,
        FloorAction::Touch {
            id: 2,
            action: TouchAction::Pressed,
            point: Point::new(60.0, 50.0),
            is_in_contact: true,
        },
    );
    let scale_before_pinch = state.transformation_matrix.scale_x;
    reduce(
        &mut state,
        FloorAction::Touch {
            id: 1,
            action: TouchAction::Moved,
            point: Point::new(30.0, 50.0),
            is_in_contact: true,
        },
    );
    reduce(
        &mut state,
        FloorAction::Touch {
            id: 2,
            action: TouchAction::Moved,
            point: Point::new(70.0, 50.0),
            is_in_contact: true,
        },
    );
    assert!(state.transformation_matrix.scale_x > scale_before_pinch);

    reduce(
        &mut state,
        FloorAction::PointerPressed {
            point: Point::new(40.0, 40.0),
        },
    );
    reduce(
        &mut state,
        FloorAction::PointerReleased {
            point: Point::new(40.0, 40.0),
        },
    );
    reduce(
        &mut state,
        FloorAction::PointerPressed {
            point: Point::new(42.0, 41.0),
        },
    );
    reduce(
        &mut state,
        FloorAction::PointerReleased {
            point: Point::new(42.0, 41.0),
        },
    );
    assert_eq!(
        state.transformation_matrix,
        crate::floor::floor_component::state::Matrix::identity()
    );
}
