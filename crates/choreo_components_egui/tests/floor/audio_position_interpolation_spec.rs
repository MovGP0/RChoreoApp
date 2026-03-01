use crate::floor::floor_component::actions::FloorAction;
use crate::floor::floor_component::reducer::reduce;
use crate::floor::floor_component::state::FloorPosition;
use crate::floor::floor_component::state::FloorState;

#[test]
fn audio_position_interpolation_updates_interpolated_positions() {
    let mut state = FloorState::default();
    reduce(
        &mut state,
        FloorAction::InterpolateAudioPosition {
            from: vec![FloorPosition::new(0.0, 0.0)],
            to: vec![FloorPosition::new(10.0, 0.0)],
            progress: 0.5,
        },
    );

    assert_eq!(state.interpolated_positions.len(), 1);
    assert!((state.interpolated_positions[0].x - 5.0).abs() < 0.0001);
    assert!((state.interpolated_positions[0].y - 0.0).abs() < 0.0001);
}
