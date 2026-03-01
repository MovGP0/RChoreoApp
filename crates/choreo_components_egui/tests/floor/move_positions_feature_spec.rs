use crate::floor::floor_component::actions::FloorAction;
use crate::floor::floor_component::reducer::reduce;
use crate::floor::floor_component::state::FloorPosition;
use crate::floor::floor_component::state::FloorState;
use crate::floor::floor_component::state::Point;

#[test]
fn move_positions_feature_supports_multi_select_single_drag_and_clear() {
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
    assert_eq!(state.selected_positions.len(), 2);

    state.selected_positions = vec![0];
    reduce(
        &mut state,
        FloorAction::MoveSelectedByDelta {
            delta_x: -1.0,
            delta_y: 2.0,
        },
    );
    assert!((state.positions[0].x + 2.0).abs() < 0.0001);
    assert!((state.positions[0].y - 3.0).abs() < 0.0001);
    assert!((state.positions[1].x - 1.0).abs() < 0.0001);

    reduce(&mut state, FloorAction::ClearSelection);
    assert!(state.selected_positions.is_empty());
    assert!(state.selection_rectangle.is_none());
}
