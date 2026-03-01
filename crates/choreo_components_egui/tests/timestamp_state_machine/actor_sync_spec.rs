use crate::timestamp_state_machine;
use timestamp_state_machine::Report;

#[test]
fn actor_sync_spec() {
    let suite = rspec::describe("timestamp actor sync", (), |spec| {
        spec.it("accepts actor position when seek target is reached", |_| {
            let mut state = timestamp_state_machine::state::TimestampStateMachineState::default();

            timestamp_state_machine::reducer::reduce(
                &mut state,
                timestamp_state_machine::actions::TimestampStateMachineAction::SeekCommitted {
                    position: 20.0,
                    now_seconds: 5.0,
                },
            );
            timestamp_state_machine::reducer::reduce(
                &mut state,
                timestamp_state_machine::actions::TimestampStateMachineAction::ActorPositionSampled {
                    position: 20.1,
                    now_seconds: 5.2,
                },
            );

            assert!(state.last_actor_sample_accepted);
            assert_eq!(
                state.ownership_phase,
                timestamp_state_machine::state::OwnershipPhase::ActorDriven
            );
            assert!(state.pending_seek_position.is_none());
        });

        spec.it("accepts actor position after timeout", |_| {
            let mut state = timestamp_state_machine::state::TimestampStateMachineState::default();

            timestamp_state_machine::reducer::reduce(
                &mut state,
                timestamp_state_machine::actions::TimestampStateMachineAction::SeekCommitted {
                    position: 30.0,
                    now_seconds: 10.0,
                },
            );
            timestamp_state_machine::reducer::reduce(
                &mut state,
                timestamp_state_machine::actions::TimestampStateMachineAction::ActorPositionSampled {
                    position: 15.0,
                    now_seconds: 12.0,
                },
            );

            assert!(state.last_actor_sample_accepted);
            assert_eq!(
                state.ownership_phase,
                timestamp_state_machine::state::OwnershipPhase::ActorDriven
            );
        });

        spec.it(
            "keeps seek pending when actor sample is out of tolerance before timeout",
            |_| {
                let mut state =
                    timestamp_state_machine::state::TimestampStateMachineState::default();

                timestamp_state_machine::reducer::reduce(
                    &mut state,
                    timestamp_state_machine::actions::TimestampStateMachineAction::SeekCommitted {
                        position: 30.0,
                        now_seconds: 10.0,
                    },
                );
                timestamp_state_machine::reducer::reduce(
                    &mut state,
                    timestamp_state_machine::actions::TimestampStateMachineAction::ActorPositionSampled {
                        position: 15.0,
                        now_seconds: 10.4,
                    },
                );

                assert!(!state.last_actor_sample_accepted);
                assert_eq!(
                    state.ownership_phase,
                    timestamp_state_machine::state::OwnershipPhase::SeekCommitPending
                );
                assert_eq!(state.pending_seek_position, Some(30.0));
            },
        );

        spec.it("rejects actor samples while user is previewing", |_| {
            let mut state = timestamp_state_machine::state::TimestampStateMachineState::default();

            timestamp_state_machine::reducer::reduce(
                &mut state,
                timestamp_state_machine::actions::TimestampStateMachineAction::DragStarted {
                    is_playing: false,
                },
            );
            timestamp_state_machine::reducer::reduce(
                &mut state,
                timestamp_state_machine::actions::TimestampStateMachineAction::ActorPositionSampled {
                    position: 8.0,
                    now_seconds: 2.0,
                },
            );

            assert!(!state.last_actor_sample_accepted);
            assert_eq!(
                state.ownership_phase,
                timestamp_state_machine::state::OwnershipPhase::UserPreview
            );
        });
    });
    let report = timestamp_state_machine::run_suite(&suite);
    assert!(report.is_success());
}
