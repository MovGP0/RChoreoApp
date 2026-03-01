use crate::timestamp_state_machine;
use timestamp_state_machine::Report;

#[test]
fn drag_and_commit_spec() {
    let suite = rspec::describe("timestamp drag and commit", (), |spec| {
        spec.it("tracks preview seek commit and resume request", |_| {
            let mut state = timestamp_state_machine::state::TimestampStateMachineState::default();

            timestamp_state_machine::reducer::reduce(
                &mut state,
                timestamp_state_machine::actions::TimestampStateMachineAction::DragStarted {
                    is_playing: true,
                },
            );
            assert_eq!(
                state.ownership_phase,
                timestamp_state_machine::state::OwnershipPhase::UserPreview
            );
            assert!(state.pause_playback_requested);

            timestamp_state_machine::reducer::reduce(
                &mut state,
                timestamp_state_machine::actions::TimestampStateMachineAction::PreviewPositionChanged {
                    position: 12.5,
                },
            );
            assert_eq!(state.preview_position, Some(12.5));

            timestamp_state_machine::reducer::reduce(
                &mut state,
                timestamp_state_machine::actions::TimestampStateMachineAction::SeekCommitted {
                    position: 12.5,
                    now_seconds: 10.0,
                },
            );
            assert_eq!(
                state.ownership_phase,
                timestamp_state_machine::state::OwnershipPhase::SeekCommitPending
            );
            assert_eq!(state.pending_seek_position, Some(12.5));

            timestamp_state_machine::reducer::reduce(
                &mut state,
                timestamp_state_machine::actions::TimestampStateMachineAction::CompleteSeekCommit,
            );
            assert_eq!(
                state.ownership_phase,
                timestamp_state_machine::state::OwnershipPhase::ActorDriven
            );
            assert!(state.resume_playback_requested);
        });

        spec.it("does not request resume when drag started while paused", |_| {
            let mut state = timestamp_state_machine::state::TimestampStateMachineState::default();

            timestamp_state_machine::reducer::reduce(
                &mut state,
                timestamp_state_machine::actions::TimestampStateMachineAction::DragStarted {
                    is_playing: false,
                },
            );
            timestamp_state_machine::reducer::reduce(
                &mut state,
                timestamp_state_machine::actions::TimestampStateMachineAction::SeekCommitted {
                    position: 4.0,
                    now_seconds: 1.0,
                },
            );
            timestamp_state_machine::reducer::reduce(
                &mut state,
                timestamp_state_machine::actions::TimestampStateMachineAction::CompleteSeekCommit,
            );

            assert!(!state.resume_playback_requested);
            assert_eq!(
                state.ownership_phase,
                timestamp_state_machine::state::OwnershipPhase::ActorDriven
            );
        });
    });
    let report = timestamp_state_machine::run_suite(&suite);
    assert!(report.is_success());
}
