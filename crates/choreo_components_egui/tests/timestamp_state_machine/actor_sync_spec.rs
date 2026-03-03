use std::thread::sleep;
use std::time::Duration;

#[test]
fn actor_sync_spec() {
    let mut machine = super::machine::TimestampOwnershipStateMachine::new();
    let _ = machine.apply_event(super::machine::TimestampEvent::SeekCommitted { position: 20.0 });

    let accepted = machine
        .apply_event(super::machine::TimestampEvent::ActorPositionSampled { position: 20.1 });
    assert!(accepted);
    assert_eq!(
        machine.snapshot().ownership_phase,
        super::machine::OwnershipPhase::ActorDriven
    );
}

#[test]
fn actor_sync_timeout_is_machine_owned() {
    let mut machine = super::machine::TimestampOwnershipStateMachine::new();
    let _ = machine.apply_event(super::machine::TimestampEvent::SeekCommitted { position: 30.0 });

    sleep(Duration::from_millis(1600));
    let accepted = machine
        .apply_event(super::machine::TimestampEvent::ActorPositionSampled { position: 15.0 });
    assert!(accepted);
    let snapshot = machine.snapshot();
    assert_eq!(snapshot.pending_seek_position, None);
    assert_eq!(
        snapshot.ownership_phase,
        super::machine::OwnershipPhase::ActorDriven
    );
}

#[test]
fn actor_sync_rejects_samples_while_previewing() {
    let mut machine = super::machine::TimestampOwnershipStateMachine::new();
    let _ = machine.apply_event(super::machine::TimestampEvent::DragStarted { is_playing: false });

    let accepted =
        machine.apply_event(super::machine::TimestampEvent::ActorPositionSampled { position: 8.0 });
    assert!(!accepted);
    assert_eq!(
        machine.snapshot().ownership_phase,
        super::machine::OwnershipPhase::UserPreview
    );
}
