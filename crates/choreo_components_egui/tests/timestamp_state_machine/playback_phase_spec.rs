#[test]
fn playback_phase_spec() {
    let mut machine = super::machine::TimestampOwnershipStateMachine::new();

    machine.set_playback_from_player(false, false);
    assert_eq!(
        machine.snapshot().playback_phase,
        super::machine::PlaybackPhase::NoMedia
    );

    machine.set_playback_from_player(true, true);
    assert_eq!(
        machine.snapshot().playback_phase,
        super::machine::PlaybackPhase::ReadyPlaying
    );

    machine.set_is_adjusting_speed(true);
    assert!(machine.is_adjusting_speed());
}
