use std::time::Duration;

use crate::scenes;

use choreo_components::behavior::Behavior;
use choreo_components::scenes::LoadScenesBehavior;
use choreo_components::scenes::ReloadScenesCommand;
use crossbeam_channel::unbounded;
use scenes::Report;

#[test]
#[serial_test::serial]
fn load_scenes_behavior_spec() {
    let suite = rspec::describe("load scenes behavior", (), |spec| {
        spec.it("loads scenes on activation and selects first scene", |_| {
            let context = scenes::ScenesTestContext::new();

            let first_model = scenes::build_scene_model(
                1,
                "Intro",
                Some("00:05"),
                vec![scenes::build_position(0.0, 0.0)],
            );
            let second_model =
                scenes::build_scene_model(2, "Verse", None, vec![scenes::build_position(1.0, 1.0)]);
            context.update_global_state(|state| {
                state.choreography.scenes = vec![first_model.clone(), second_model];
            });

            let (_reload_sender, reload_receiver) = unbounded::<ReloadScenesCommand>();
            let (selected_scene_changed_sender, selected_scene_changed_receiver) = unbounded();
            let behavior = LoadScenesBehavior::new(
                context.global_state_store.clone(),
                reload_receiver,
                selected_scene_changed_sender,
            );
            context.activate_behaviors(vec![Box::new(behavior) as Box<dyn Behavior<_>>]);

            let loaded = context.wait_until(Duration::from_secs(1), || {
                context.read_global_state(|state| {
                    state.scenes.len() == 2
                        && state.selected_scene.as_ref().map(|scene| scene.scene_id)
                            == Some(first_model.scene_id)
                })
            });
            assert!(loaded);

            let selected_event = selected_scene_changed_receiver
                .try_recv()
                .expect("selected scene changed event should be emitted");
            assert_eq!(
                selected_event
                    .selected_scene
                    .as_ref()
                    .map(|scene| scene.scene_id),
                Some(first_model.scene_id)
            );

            let selected_vm = context
                .view_model
                .borrow()
                .selected_scene()
                .expect("view model selected scene should be set");
            assert_eq!(selected_vm.scene_id, first_model.scene_id);
            assert_eq!(selected_vm.name, "Intro");
            assert_eq!(selected_vm.timestamp, Some(5.0));
        });

        spec.it("reloads scenes when a reload command is received", |_| {
            let context = scenes::ScenesTestContext::new();

            let first_model = scenes::build_scene_model(1, "First", None, vec![]);
            context.update_global_state(|state| {
                state.choreography.scenes = vec![first_model.clone()];
            });

            let (reload_sender, reload_receiver) = unbounded::<ReloadScenesCommand>();
            let (selected_scene_changed_sender, _selected_scene_changed_receiver) = unbounded();
            let behavior = LoadScenesBehavior::new(
                context.global_state_store.clone(),
                reload_receiver,
                selected_scene_changed_sender,
            );
            context.activate_behaviors(vec![Box::new(behavior) as Box<dyn Behavior<_>>]);

            let second_model = scenes::build_scene_model(2, "Second", Some("00:09"), vec![]);
            context.update_global_state(|state| {
                state.choreography.scenes = vec![first_model.clone(), second_model.clone()];
            });

            reload_sender
                .send(ReloadScenesCommand)
                .expect("reload send should succeed");

            let reloaded = context.wait_until(Duration::from_secs(1), || {
                context.read_global_state(|state| state.scenes.len() == 2)
            });
            assert!(reloaded);

            let scenes = context.read_global_state(|state| state.scenes.clone());
            assert_eq!(scenes[0].scene_id, first_model.scene_id);
            assert_eq!(scenes[1].scene_id, second_model.scene_id);
            assert_eq!(scenes[1].timestamp, Some(9.0));
        });
    });

    let report = scenes::run_suite(&suite);
    assert!(report.is_success());
}
