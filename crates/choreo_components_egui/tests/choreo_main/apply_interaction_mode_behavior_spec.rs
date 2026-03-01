use crate::choreo_main::Report;
use crate::choreo_main::actions::ChoreoMainAction;
use crate::choreo_main::reducer::reduce;
use crate::choreo_main::state::ChoreoMainState;
use crate::choreo_main::state::InteractionMode;
use crate::choreo_main::state::InteractionStateMachineState;

#[test]
fn apply_interaction_mode_behavior_spec() {
    let suite = rspec::describe("apply interaction mode reducer behavior", (), |spec| {
        spec.it("switches interaction state to move mode", |_| {
            let mut state = ChoreoMainState::default();
            reduce(&mut state, ChoreoMainAction::Initialize);
            reduce(
                &mut state,
                ChoreoMainAction::ApplyInteractionMode {
                    mode: InteractionMode::Move,
                    selected_positions_count: 0,
                },
            );

            assert_eq!(state.interaction_mode, InteractionMode::Move);
            assert_eq!(
                state.interaction_state_machine,
                InteractionStateMachineState::MovePositions
            );
        });

        spec.it("maps rotate and scale modes with and without selection", |_| {
            let mut state = ChoreoMainState::default();

            reduce(
                &mut state,
                ChoreoMainAction::ApplyInteractionMode {
                    mode: InteractionMode::RotateAroundCenter,
                    selected_positions_count: 0,
                },
            );
            assert_eq!(
                state.interaction_state_machine,
                InteractionStateMachineState::RotateAroundCenter
            );

            reduce(
                &mut state,
                ChoreoMainAction::ApplyInteractionMode {
                    mode: InteractionMode::RotateAroundDancer,
                    selected_positions_count: 1,
                },
            );
            assert_eq!(
                state.interaction_state_machine,
                InteractionStateMachineState::ScaleAroundDancerSelection
            );

            reduce(
                &mut state,
                ChoreoMainAction::ApplyInteractionMode {
                    mode: InteractionMode::Scale,
                    selected_positions_count: 0,
                },
            );
            assert_eq!(
                state.interaction_state_machine,
                InteractionStateMachineState::ScalePositions
            );
        });
    });

    let report = crate::choreo_main::run_suite(&suite);
    assert!(report.is_success());
}
