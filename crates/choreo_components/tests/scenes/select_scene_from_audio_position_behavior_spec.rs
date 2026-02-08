use std::time::Duration;

use crate::scenes;

use choreo_components::audio_player::AudioPlayerPositionChangedEvent;
use choreo_components::behavior::Behavior;
use choreo_components::scenes::SelectSceneFromAudioPositionBehavior;
use crossbeam_channel::unbounded;
use scenes::Report;

#[test]
#[serial_test::serial]
fn select_scene_from_audio_position_behavior_spec() {
    let suite = rspec::describe("select scene from audio position behavior", (), |spec| {
        spec.it("selects scene whose timestamp range contains current position", |_| {
            let context = scenes::ScenesTestContext::new();

            let first = scenes::build_scene_model(1, "First", Some("00:05"), vec![]);
            let second = scenes::build_scene_model(2, "Second", Some("00:10"), vec![]);
            let third = scenes::build_scene_model(3, "Third", Some("00:20"), vec![]);
            let first_vm = scenes::map_scene_view_model(&first);
            let second_vm = scenes::map_scene_view_model(&second);
            let third_vm = scenes::map_scene_view_model(&third);
            context.update_global_state(|state| {
                state.scenes = vec![first_vm.clone(), second_vm.clone(), third_vm.clone()];
                state.selected_scene = Some(first_vm.clone());
            });
            context.view_model.borrow_mut().refresh_scenes();
            context.view_model.borrow_mut().set_selected_scene(Some(first_vm));

            let (position_sender, position_receiver) = unbounded::<AudioPlayerPositionChangedEvent>();
            let (selected_scene_changed_sender, selected_scene_changed_receiver) = unbounded();
            let behavior = SelectSceneFromAudioPositionBehavior::new(
                position_receiver,
                selected_scene_changed_sender,
            );
            context.activate_behaviors(vec![Box::new(behavior) as Box<dyn Behavior<_>>]);

            position_sender
                .send(AudioPlayerPositionChangedEvent { position_seconds: 12.0 })
                .expect("position send should succeed");

            let selected = context.wait_until(Duration::from_secs(1), || {
                context
                    .read_global_state(|state| state.selected_scene.as_ref().map(|scene| scene.scene_id))
                    == Some(second.scene_id)
            });
            assert!(selected);

            let event = selected_scene_changed_receiver.try_recv().expect("selection change event should be emitted");
            assert_eq!(event.selected_scene.as_ref().map(|scene| scene.scene_id), Some(second.scene_id));
        });

        spec.it("does not emit event when computed scene does not change", |_| {
            let context = scenes::ScenesTestContext::new();

            let first = scenes::build_scene_model(1, "First", Some("00:05"), vec![]);
            let second = scenes::build_scene_model(2, "Second", Some("00:10"), vec![]);
            let first_vm = scenes::map_scene_view_model(&first);
            let second_vm = scenes::map_scene_view_model(&second);
            context.update_global_state(|state| {
                state.scenes = vec![first_vm.clone(), second_vm];
                state.selected_scene = Some(first_vm.clone());
            });
            context.view_model.borrow_mut().refresh_scenes();
            context.view_model.borrow_mut().set_selected_scene(Some(first_vm));

            let (position_sender, position_receiver) = unbounded::<AudioPlayerPositionChangedEvent>();
            let (selected_scene_changed_sender, selected_scene_changed_receiver) = unbounded();
            let behavior = SelectSceneFromAudioPositionBehavior::new(
                position_receiver,
                selected_scene_changed_sender,
            );
            context.activate_behaviors(vec![Box::new(behavior) as Box<dyn Behavior<_>>]);

            position_sender
                .send(AudioPlayerPositionChangedEvent { position_seconds: 7.0 })
                .expect("position send should succeed");
            context.pump_events();

            assert!(selected_scene_changed_receiver.try_recv().is_err());
            assert_eq!(
                context.read_global_state(|state| state.selected_scene.as_ref().map(|scene| scene.scene_id)),
                Some(first.scene_id)
            );
        });
    });

    let report = scenes::run_suite(&suite);
    assert!(report.is_success());
}
