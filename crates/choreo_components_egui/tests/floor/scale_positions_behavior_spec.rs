use crate::floor::floor_component::actions::FloorAction;
use crate::floor::floor_component::reducer::reduce;
use crate::floor::floor_component::state::FloorPosition;
use crate::floor::floor_component::state::FloorState;
use crate::floor::floor_component::state::Point;

#[test]
fn scale_positions_scales_selected_positions_from_center() {
    let mut state = FloorState::default();
    reduce(
        &mut state,
        FloorAction::SetPositions {
            positions: vec![
                FloorPosition::new(-1.0, 1.0),
                FloorPosition::new(1.0, 1.0),
                FloorPosition::new(3.0, -2.0),
            ],
        },
    );
    reduce(
        &mut state,
        FloorAction::SelectRectangle {
            start: Point::new(-2.0, 2.0),
            end: Point::new(2.0, 0.0),
        },
    );
    reduce(
        &mut state,
        FloorAction::ScaleSelected {
            start: Point::new(2.0, 1.0),
            end: Point::new(4.0, 1.0),
        },
    );

    assert!((state.positions[0].x + 2.0).abs() < 0.0001);
    assert!((state.positions[0].y - 1.0).abs() < 0.0001);
    assert!((state.positions[1].x - 2.0).abs() < 0.0001);
    assert!((state.positions[1].y - 1.0).abs() < 0.0001);
    assert!((state.positions[2].x - 3.0).abs() < 0.0001);
    assert!((state.positions[2].y + 2.0).abs() < 0.0001);
}
