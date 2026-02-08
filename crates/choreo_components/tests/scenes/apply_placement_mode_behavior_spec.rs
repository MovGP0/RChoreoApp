use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use crate::scenes;

use choreo_components::behavior::Behavior;
use choreo_components::scenes::ApplyPlacementModeBehavior;
use choreo_components::scenes::SelectedSceneChangedEvent;
use choreo_state_machine::ApplicationStateMachine;
use choreo_state_machine::StateKind;
use crossbeam_channel::unbounded;
use scenes::Report;

#[test]
#[serial_test::serial]
fn apply_placement_mode_behavior_spec() {
    let suite = rspec::describe("apply placement mode behavior", (), |spec| {
        spec.it("enables place mode when selected scene has fewer positions than dancers", |_| {
            let context = scenes::ScenesTestContext::new();

            let first = scenes::build_scene_model(1, "First", None, vec![scenes::build_position(0.0, 0.0)]);
            let first_vm = scenes::map_scene_view_model(&first);
            let dancer_one = scenes::build_dancer(1, "A");
            let dancer_two = scenes::build_dancer(2, "B");
            context.update_global_state(|state| {
                state.choreography.scenes = vec![first.clone()];
                state.choreography.dancers = vec![dancer_one, dancer_two];
                state.selected_scene = Some(first_vm.clone());
            });

            let state_machine = Rc::new(RefCell::new(ApplicationStateMachine::with_default_transitions(Box::new(
                choreo_components::global::GlobalStateModel::default(),
            ))));
            let (_selected_scene_changed_sender, selected_scene_changed_receiver) = unbounded::<SelectedSceneChangedEvent>();
            let behavior = ApplyPlacementModeBehavior::new(
                context.global_state_store.clone(),
                Some(state_machine.clone()),
                selected_scene_changed_receiver,
            );
            context.activate_behaviors(vec![Box::new(behavior) as Box<dyn Behavior<_>>]);

            let applied = context.wait_until(Duration::from_secs(1), || {
                context.read_global_state(|state| state.is_place_mode)
            });
            assert!(applied);
            assert_eq!(state_machine.borrow().state().kind(), StateKind::PlacePositionsState);
        });

        spec.it("disables place mode when selection is cleared", |_| {
            let context = scenes::ScenesTestContext::new();

            let first = scenes::build_scene_model(1, "First", None, vec![]);
            context.update_global_state(|state| {
                state.choreography.scenes = vec![first.clone()];
                state.choreography.dancers = vec![scenes::build_dancer(1, "A")];
                state.selected_scene = Some(scenes::map_scene_view_model(&first));
                state.is_place_mode = true;
            });

            let state_machine = Rc::new(RefCell::new(ApplicationStateMachine::with_default_transitions(Box::new(
                choreo_components::global::GlobalStateModel::default(),
            ))));
            let (selected_scene_changed_sender, selected_scene_changed_receiver) = unbounded::<SelectedSceneChangedEvent>();
            let behavior = ApplyPlacementModeBehavior::new(
                context.global_state_store.clone(),
                Some(state_machine.clone()),
                selected_scene_changed_receiver,
            );
            context.activate_behaviors(vec![Box::new(behavior) as Box<dyn Behavior<_>>]);

            selected_scene_changed_sender
                .send(SelectedSceneChangedEvent { selected_scene: None })
                .expect("send should succeed");

            let cleared = context.wait_until(Duration::from_secs(1), || {
                !context.read_global_state(|state| state.is_place_mode)
            });
            assert!(cleared);
            assert_eq!(state_machine.borrow().state().kind(), StateKind::ViewSceneState);
        });
    });

    let report = scenes::run_suite(&suite);
    assert!(report.is_success());
}
