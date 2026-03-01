use crate::timestamp_state_machine;
use timestamp_state_machine::Report;

#[test]
fn playback_phase_spec() {
    let suite = rspec::describe("timestamp playback phase", (), |spec| {
        spec.it("maps player state into playback phase and tracks speed adjustment", |_| {
            let mut state = timestamp_state_machine::state::TimestampStateMachineState::default();
            timestamp_state_machine::reducer::reduce(
                &mut state,
                timestamp_state_machine::actions::TimestampStateMachineAction::Initialize,
            );

            timestamp_state_machine::reducer::reduce(
                &mut state,
                timestamp_state_machine::actions::TimestampStateMachineAction::SetPlaybackFromPlayer {
                    has_player: false,
                    is_playing: false,
                },
            );
            assert_eq!(
                state.playback_phase,
                timestamp_state_machine::state::PlaybackPhase::NoMedia
            );

            timestamp_state_machine::reducer::reduce(
                &mut state,
                timestamp_state_machine::actions::TimestampStateMachineAction::SetPlaybackFromPlayer {
                    has_player: true,
                    is_playing: true,
                },
            );
            assert_eq!(
                state.playback_phase,
                timestamp_state_machine::state::PlaybackPhase::ReadyPlaying
            );

            timestamp_state_machine::reducer::reduce(
                &mut state,
                timestamp_state_machine::actions::TimestampStateMachineAction::SetIsAdjustingSpeed {
                    value: true,
                },
            );
            assert!(state.is_adjusting_speed);
        });
    });
    let report = timestamp_state_machine::run_suite(&suite);
    assert!(report.is_success());
}
