use crate::floor::floor_component::actions::FloorAction;
use crate::floor::floor_component::reducer::reduce;
use crate::floor::floor_component::state::FloorPosition;
use crate::floor::floor_component::state::FloorState;
use crate::floor::floor_component::state::Point;

#[test]
fn move_positions_moves_selected_items_and_supports_grid_snap() {
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
        FloorAction::MoveSelectedByDelta {
            delta_x: 1.5,
            delta_y: -1.0,
        },
    );

    assert!((state.positions[0].x - 0.5).abs() < 0.0001);
    assert!((state.positions[0].y - 0.0).abs() < 0.0001);
    assert!((state.positions[1].x - 2.5).abs() < 0.0001);
    assert!((state.positions[1].y - 0.0).abs() < 0.0001);
    assert!((state.positions[2].x - 3.0).abs() < 0.0001);
    assert!((state.positions[2].y + 2.0).abs() < 0.0001);

    reduce(
        &mut state,
        FloorAction::SetSnapToGrid {
            enabled: true,
            resolution: 4,
        },
    );
    reduce(
        &mut state,
        FloorAction::MoveSelectedByDelta {
            delta_x: -0.3,
            delta_y: 0.0,
        },
    );

    assert!((state.positions[0].x - 0.25).abs() < 0.0001);
    assert!((state.positions[1].x - 2.25).abs() < 0.0001);
}
