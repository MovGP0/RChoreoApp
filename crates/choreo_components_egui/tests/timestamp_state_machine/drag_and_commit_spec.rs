#[test]
fn drag_and_commit_spec() {
    let mut machine = super::machine::TimestampOwnershipStateMachine::new();

    let pause_requested = machine.apply_event(super::machine::TimestampEvent::DragStarted {
        is_playing: true,
    });
    assert!(pause_requested);
    assert_eq!(
        machine.snapshot().ownership_phase,
        super::machine::OwnershipPhase::UserPreview
    );

    let _ = machine.apply_event(super::machine::TimestampEvent::PreviewPositionChanged {
        position: 12.5,
    });
    let _ = machine.apply_event(super::machine::TimestampEvent::SeekCommitted { position: 12.5 });
    assert_eq!(
        machine.snapshot().ownership_phase,
        super::machine::OwnershipPhase::SeekCommitPending
    );
    assert_eq!(machine.snapshot().pending_seek_position, Some(12.5));

    let resume_requested = machine.complete_seek_commit();
    assert!(resume_requested);
    assert_eq!(
        machine.snapshot().ownership_phase,
        super::machine::OwnershipPhase::ActorDriven
    );
}
