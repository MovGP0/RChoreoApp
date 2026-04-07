use crate::choreo_main::Report;
use crate::choreo_main::actions::ChoreoMainAction;
use crate::choreo_main::reducer::reduce;
use crate::choreo_main::state::ChoreoMainState;
use crate::choreo_main::state::InteractionMode;
use crate::choreo_main::state::InteractionStateMachineState;

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

fn assert_no_errors(errors: Vec<String>) {
    assert!(
        errors.is_empty(),
        "Assertion failures:\n{}",
        errors.join("\n")
    );
}

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

            let mut errors = Vec::new();

            check_eq!(errors, state.interaction_mode, InteractionMode::Move);
            check_eq!(
                errors,
                state.interaction_state_machine,
                InteractionStateMachineState::MovePositions
            );

            assert_no_errors(errors);
        });

        spec.it(
            "maps rotate and scale modes with and without selection",
            |_| {
                let mut state = ChoreoMainState::default();

                reduce(
                    &mut state,
                    ChoreoMainAction::ApplyInteractionMode {
                        mode: InteractionMode::RotateAroundCenter,
                        selected_positions_count: 0,
                    },
                );

                let mut errors = Vec::new();

                check_eq!(
                    errors,
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
                check_eq!(
                    errors,
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
                check_eq!(
                    errors,
                    state.interaction_state_machine,
                    InteractionStateMachineState::ScalePositions
                );

                assert_no_errors(errors);
            },
        );
    });

    let report = crate::choreo_main::run_suite(&suite);
    assert!(report.is_success());
}
