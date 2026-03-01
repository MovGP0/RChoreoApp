use crate::floor::floor_component::actions::FloorAction;
use crate::floor::floor_component::reducer::reduce;
use crate::floor::floor_component::state::FloorState;

#[test]
fn redraw_floor_increments_draw_counter() {
    let mut state = FloorState::default();
    reduce(&mut state, FloorAction::RedrawFloor);
    reduce(&mut state, FloorAction::RedrawFloor);
    reduce(&mut state, FloorAction::RedrawFloor);

    assert_eq!(state.draw_count, 3);
}
