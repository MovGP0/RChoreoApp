use std::thread::sleep;
use std::time::Duration;

macro_rules! check_eq {
    ($errors:expr, $left:expr, $right:expr) => {
        if $left != $right {
            $errors.push(format!(
                "{} != {} (left = {:?}, right = {:?})",
                stringify!($left),
                stringify!($right),
                $left,
                $right
            ));
        }
    };
}

macro_rules! check {
    ($errors:expr, $condition:expr) => {
        if !$condition {
            $errors.push(format!("condition failed: {}", stringify!($condition)));
        }
    };
}

fn assert_no_errors(errors: Vec<String>) {
    assert!(
        errors.is_empty(),
        "Assertion failures:\n{}",
        errors.join("\n")
    );
}

#[test]
fn actor_sync_spec() {
    let mut machine = super::machine::TimestampOwnershipStateMachine::new();
    let _ = machine.apply_event(super::machine::TimestampEvent::SeekCommitted { position: 20.0 });

    let accepted = machine
        .apply_event(super::machine::TimestampEvent::ActorPositionSampled { position: 20.1 });
    let mut errors = Vec::new();
    check!(errors, accepted);
    check_eq!(
        errors,
        machine.snapshot().ownership_phase,
        super::machine::OwnershipPhase::ActorDriven
    );

    assert_no_errors(errors);
}

#[test]
fn actor_sync_timeout_is_machine_owned() {
    let mut machine = super::machine::TimestampOwnershipStateMachine::new();
    let _ = machine.apply_event(super::machine::TimestampEvent::SeekCommitted { position: 30.0 });

    sleep(Duration::from_millis(1600));
    let accepted = machine
        .apply_event(super::machine::TimestampEvent::ActorPositionSampled { position: 15.0 });
    let snapshot = machine.snapshot();
    let mut errors = Vec::new();
    check!(errors, accepted);
    check_eq!(errors, snapshot.pending_seek_position, None::<f64>);
    check_eq!(
        errors,
        snapshot.ownership_phase,
        super::machine::OwnershipPhase::ActorDriven
    );

    assert_no_errors(errors);
}

#[test]
fn actor_sync_rejects_samples_while_previewing() {
    let mut machine = super::machine::TimestampOwnershipStateMachine::new();
    let _ = machine.apply_event(super::machine::TimestampEvent::DragStarted { is_playing: false });

    let accepted =
        machine.apply_event(super::machine::TimestampEvent::ActorPositionSampled { position: 8.0 });
    let mut errors = Vec::new();
    check!(errors, !accepted);
    check_eq!(
        errors,
        machine.snapshot().ownership_phase,
        super::machine::OwnershipPhase::UserPreview
    );

    assert_no_errors(errors);
}
