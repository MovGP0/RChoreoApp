use crate::floor::floor_component::actions::FloorAction;
use crate::floor::floor_component::reducer::reduce;
use crate::floor::floor_component::state::FloorState;

#[test]
fn draw_floor_marks_render_gate_only_once_while_counting_draws() {
    let mut state = FloorState::default();

    reduce(&mut state, FloorAction::DrawFloor);
    reduce(&mut state, FloorAction::DrawFloor);

    assert_eq!(state.draw_count, 2);
    assert!(state.render_marked);
    assert_eq!(state.render_mark_count, 1);
}
