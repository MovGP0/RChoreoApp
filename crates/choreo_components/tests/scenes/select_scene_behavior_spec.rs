use std::time::Duration;

use crate::scenes;

use choreo_components::behavior::Behavior;
use choreo_components::scenes::SelectSceneBehavior;
use choreo_components::scenes::SelectSceneCommand;
use crossbeam_channel::unbounded;
use scenes::Report;

#[test]
#[serial_test::serial]
fn select_scene_behavior_spec() {
    let suite = rspec::describe("select scene behavior", (), |spec| {
        spec.it("selects scene, emits selected-scene event, and triggers floor redraw", |_| {
            let context = scenes::ScenesTestContext::new();

            let first = scenes::build_scene_model(1, "First", None, vec![]);
            let second = scenes::build_scene_model(2, "Second", None, vec![]);
            context.update_global_state(|state| {
                state.scenes = vec![
                    scenes::map_scene_view_model(&first),
                    scenes::map_scene_view_model(&second),
                ];
                state.selected_scene = state.scenes.first().cloned();
            });
            context.view_model.borrow_mut().refresh_scenes();
            context
                .view_model
                .borrow_mut()
                .set_selected_scene(context.read_global_state(|state| state.selected_scene.clone()));

            let (select_scene_sender, select_scene_receiver) = unbounded::<SelectSceneCommand>();
            let (selected_scene_changed_sender, selected_scene_changed_receiver) = unbounded();
            let (redraw_floor_sender, redraw_floor_receiver) = unbounded();
            let behavior = SelectSceneBehavior::new(
                select_scene_sender,
                select_scene_receiver,
                selected_scene_changed_sender,
                redraw_floor_sender,
            );
            context.activate_behaviors(vec![Box::new(behavior) as Box<dyn Behavior<_>>]);

            context.view_model.borrow_mut().select_scene(1);

            let selected = context.wait_until(Duration::from_secs(1), || {
                context.read_global_state(|state| {
                    state
                        .selected_scene
                        .as_ref()
                        .map(|scene| scene.scene_id)
                        == Some(second.scene_id)
                })
            });
            assert!(selected);

            let event = selected_scene_changed_receiver
                .try_recv()
                .expect("selected scene changed event should be emitted");
            assert_eq!(event.selected_scene.as_ref().map(|scene| scene.scene_id), Some(second.scene_id));
            assert!(redraw_floor_receiver.try_recv().is_ok());
        });

        spec.it("ignores out-of-range selection indices", |_| {
            let context = scenes::ScenesTestContext::new();

            let first = scenes::build_scene_model(1, "First", None, vec![]);
            context.update_global_state(|state| {
                state.scenes = vec![scenes::map_scene_view_model(&first)];
                state.selected_scene = state.scenes.first().cloned();
            });
            context.view_model.borrow_mut().refresh_scenes();
            context
                .view_model
                .borrow_mut()
                .set_selected_scene(context.read_global_state(|state| state.selected_scene.clone()));

            let (select_scene_sender, select_scene_receiver) = unbounded::<SelectSceneCommand>();
            let (selected_scene_changed_sender, selected_scene_changed_receiver) = unbounded();
            let (redraw_floor_sender, redraw_floor_receiver) = unbounded();
            let behavior = SelectSceneBehavior::new(
                select_scene_sender,
                select_scene_receiver,
                selected_scene_changed_sender,
                redraw_floor_sender,
            );
            context.activate_behaviors(vec![Box::new(behavior) as Box<dyn Behavior<_>>]);

            context.view_model.borrow_mut().select_scene(10);
            context.pump_events();

            assert_eq!(
                context.read_global_state(|state| state.selected_scene.as_ref().map(|scene| scene.scene_id)),
                Some(first.scene_id)
            );
            assert!(selected_scene_changed_receiver.try_recv().is_err());
            assert!(redraw_floor_receiver.try_recv().is_err());
        });
    });

    let report = scenes::run_suite(&suite);
    assert!(report.is_success());
}
