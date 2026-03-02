use crate::floor::floor_component::actions::FloorAction;
use crate::floor::floor_component::reducer::reduce;
use crate::floor::floor_component::state::FloorPosition;
use crate::floor::floor_component::state::FloorState;
use crate::floor::floor_component::state::Point;

#[test]
fn redraws_when_floor_positions_change() {
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
    reduce(&mut state, FloorAction::RedrawFloor);

    assert_eq!(state.draw_count, 1);
}

#[test]
fn redraws_when_selection_changes() {
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
    reduce(&mut state, FloorAction::RedrawFloor);

    assert_eq!(state.selected_positions, vec![0, 1]);
    assert_eq!(state.draw_count, 1);
}

#[test]
fn redraws_when_redraw_action_is_dispatched() {
    let mut state = FloorState::default();

    reduce(&mut state, FloorAction::RedrawFloor);

    assert_eq!(state.draw_count, 1);
}
