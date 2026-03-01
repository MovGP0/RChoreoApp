use crate::floor::floor_component::actions::FloorAction;
use crate::floor::floor_component::reducer::reduce;
use crate::floor::floor_component::state::FloorState;
use crate::floor::floor_component::state::Point;

#[test]
fn place_position_adds_new_position_at_click_location() {
    let mut state = FloorState {
        is_place_mode: true,
        ..FloorState::default()
    };
    reduce(
        &mut state,
        FloorAction::PlacePosition {
            point: Point::new(1.0, 1.0),
        },
    );

    assert_eq!(state.positions.len(), 1);
    assert!((state.positions[0].x - 1.0).abs() < 0.0001);
    assert!((state.positions[0].y - 1.0).abs() < 0.0001);
}
