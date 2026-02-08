use crate::scenes;

use choreo_components::behavior::Behavior;
use choreo_components::scenes::FilterScenesBehavior;
use scenes::Report;

#[test]
#[serial_test::serial]
fn filter_scenes_behavior_spec() {
    let suite = rspec::describe("filter scenes behavior", (), |spec| {
        spec.it("filters scenes by search text", |_| {
            let context = scenes::ScenesTestContext::new();

            let first = scenes::build_scene_model(1, "Opening", None, vec![]);
            let second = scenes::build_scene_model(2, "Chorus", None, vec![]);
            let third = scenes::build_scene_model(3, "Closing", None, vec![]);
            context.update_global_state(|state| {
                state.scenes = vec![
                    scenes::map_scene_view_model(&first),
                    scenes::map_scene_view_model(&second),
                    scenes::map_scene_view_model(&third),
                ];
            });
            context.view_model.borrow_mut().refresh_scenes();

            let behavior = FilterScenesBehavior;
            context.activate_behaviors(vec![Box::new(behavior) as Box<dyn Behavior<_>>]);

            context
                .view_model
                .borrow_mut()
                .update_search_text("clo".to_string());
            context.pump_events();

            let visible = context.view_model.borrow().scenes.clone();
            assert_eq!(visible.len(), 1);
            assert_eq!(visible[0].name, "Closing");
        });

        spec.it("restores all scenes when search text is cleared", |_| {
            let context = scenes::ScenesTestContext::new();

            let first = scenes::build_scene_model(1, "Opening", None, vec![]);
            let second = scenes::build_scene_model(2, "Chorus", None, vec![]);
            context.update_global_state(|state| {
                state.scenes = vec![
                    scenes::map_scene_view_model(&first),
                    scenes::map_scene_view_model(&second),
                ];
            });
            context.view_model.borrow_mut().refresh_scenes();

            let behavior = FilterScenesBehavior;
            context.activate_behaviors(vec![Box::new(behavior) as Box<dyn Behavior<_>>]);

            context
                .view_model
                .borrow_mut()
                .update_search_text("open".to_string());
            context.pump_events();
            assert_eq!(context.view_model.borrow().scenes.len(), 1);

            context
                .view_model
                .borrow_mut()
                .update_search_text(String::new());
            context.pump_events();

            assert_eq!(context.view_model.borrow().scenes.len(), 2);
        });
    });

    let report = scenes::run_suite(&suite);
    assert!(report.is_success());
}
