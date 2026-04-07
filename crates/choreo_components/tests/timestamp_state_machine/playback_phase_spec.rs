#[test]
fn playback_phase_spec() {
    let mut machine = super::machine::TimestampOwnershipStateMachine::new();

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

    let mut errors = Vec::new();

    machine.set_playback_from_player(false, false);
    check_eq!(
        errors,
        machine.snapshot().playback_phase,
        super::machine::PlaybackPhase::NoMedia
    );

    machine.set_playback_from_player(true, true);
    check_eq!(
        errors,
        machine.snapshot().playback_phase,
        super::machine::PlaybackPhase::ReadyPlaying
    );

    machine.set_is_adjusting_speed(true);
    check!(errors, machine.is_adjusting_speed());

    assert!(
        errors.is_empty(),
        "Assertion failures:\n{}",
        errors.join("\n")
    );
}
