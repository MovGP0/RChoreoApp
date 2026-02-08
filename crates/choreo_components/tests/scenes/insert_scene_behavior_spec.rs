use crate::scenes;

use choreo_components::behavior::Behavior;
use choreo_components::scenes::InsertSceneBehavior;
use scenes::Report;

#[test]
#[serial_test::serial]
fn insert_scene_behavior_spec() {
    let suite = rspec::describe("insert scene behavior", (), |spec| {
        spec.it(
            "inserts a scene after the selected one and selects it",
            |_| {
                let context = scenes::ScenesTestContext::new();

                let first = scenes::build_scene_model(1, "First", None, vec![]);
                let second = scenes::build_scene_model(2, "Second", None, vec![]);
                context.update_global_state(|state| {
                    state.choreography.scenes = vec![first.clone(), second.clone()];
                    state.scenes = vec![
                        scenes::map_scene_view_model(&first),
                        scenes::map_scene_view_model(&second),
                    ];
                    state.selected_scene = state.scenes.first().cloned();
                });
                context.view_model.borrow_mut().refresh_scenes();
                context.view_model.borrow_mut().set_selected_scene(
                    context.read_global_state(|state| state.selected_scene.clone()),
                );

                let behavior = InsertSceneBehavior::new(context.global_state_store.clone());
                context.activate_behaviors(vec![Box::new(behavior) as Box<dyn Behavior<_>>]);

                context.view_model.borrow_mut().add_scene_after();
                context.pump_events();

                let scenes = context.read_global_state(|state| state.scenes.clone());
                assert_eq!(scenes.len(), 3);
                assert_eq!(scenes[0].scene_id, first.scene_id);
                assert_eq!(scenes[1].name, "New Scene");
                assert_eq!(scenes[2].scene_id, second.scene_id);

                let selected = context
                    .read_global_state(|state| state.selected_scene.clone())
                    .expect("selected scene should exist");
                assert_eq!(selected.scene_id, scenes[1].scene_id);
                assert_eq!(
                    context.read_global_state(|state| state.choreography.scenes.len()),
                    3
                );
            },
        );

        spec.it("inserts at the end when no scene is selected", |_| {
            let context = scenes::ScenesTestContext::new();

            let first = scenes::build_scene_model(1, "First", None, vec![]);
            context.update_global_state(|state| {
                state.choreography.scenes = vec![first.clone()];
                state.scenes = vec![scenes::map_scene_view_model(&first)];
                state.selected_scene = None;
            });
            context.view_model.borrow_mut().refresh_scenes();
            context.view_model.borrow_mut().set_selected_scene(None);

            let behavior = InsertSceneBehavior::new(context.global_state_store.clone());
            context.activate_behaviors(vec![Box::new(behavior) as Box<dyn Behavior<_>>]);

            context.view_model.borrow_mut().add_scene_before();
            context.pump_events();

            let scenes = context.read_global_state(|state| state.scenes.clone());
            assert_eq!(scenes.len(), 2);
            assert_eq!(scenes[1].name, "New Scene");
            assert_eq!(
                context.read_global_state(|state| state.choreography.scenes.len()),
                2
            );
        });
    });

    let report = scenes::run_suite(&suite);
    assert!(report.is_success());
}
