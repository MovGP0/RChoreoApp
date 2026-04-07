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
fn drag_and_commit_spec() {
    let mut machine = super::machine::TimestampOwnershipStateMachine::new();
    let mut errors = Vec::new();

    let pause_requested =
        machine.apply_event(super::machine::TimestampEvent::DragStarted { is_playing: true });
    check!(errors, pause_requested);
    check_eq!(
        errors,
        machine.snapshot().ownership_phase,
        super::machine::OwnershipPhase::UserPreview
    );

    let _ = machine
        .apply_event(super::machine::TimestampEvent::PreviewPositionChanged { position: 12.5 });
    let _ = machine.apply_event(super::machine::TimestampEvent::SeekCommitted { position: 12.5 });
    check_eq!(
        errors,
        machine.snapshot().ownership_phase,
        super::machine::OwnershipPhase::SeekCommitPending
    );
    check_eq!(errors, machine.snapshot().pending_seek_position, Some(12.5));

    let resume_requested = machine.complete_seek_commit();
    check!(errors, resume_requested);
    check_eq!(
        errors,
        machine.snapshot().ownership_phase,
        super::machine::OwnershipPhase::ActorDriven
    );

    assert_no_errors(errors);
}
