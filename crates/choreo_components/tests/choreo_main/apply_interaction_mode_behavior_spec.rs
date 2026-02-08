use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use crate::choreo_main;

use choreo_components::behavior::Behavior;
use choreo_components::choreo_main::ApplyInteractionModeBehavior;
use choreo_components::global::InteractionMode;
use choreo_main::Report;
use choreo_state_machine::StateKind;
use crossbeam_channel::unbounded;

#[test]
#[serial_test::serial]
fn apply_interaction_mode_behavior_spec() {
    let suite = rspec::describe("apply interaction mode behavior", (), |spec| {
        spec.it("switches state machine to move mode", |_| {
            let (sender, receiver) = unbounded::<InteractionMode>();
            let global_state_store = choreo_components::global::GlobalStateActor::new();
            let state_machine = Rc::new(RefCell::new(
                choreo_state_machine::ApplicationStateMachine::with_default_transitions(Box::new(
                    choreo_components::global::GlobalStateModel::default(),
                )),
            ));
            let behavior = ApplyInteractionModeBehavior::new(
                global_state_store.clone(),
                state_machine.clone(),
                receiver,
            );

            let context = choreo_main::ChoreoMainTestContext::with_dependencies(
                vec![Box::new(behavior) as Box<dyn Behavior<_>>],
                global_state_store,
                state_machine,
            );

            sender
                .send(InteractionMode::Move)
                .expect("send should succeed");

            let applied = context.wait_until(Duration::from_secs(1), || {
                context.state_machine.borrow().state().kind() == StateKind::MovePositionsState
            });
            assert!(applied);
        });
    });

    let report = choreo_main::run_suite(&suite);
    assert!(report.is_success());
}
