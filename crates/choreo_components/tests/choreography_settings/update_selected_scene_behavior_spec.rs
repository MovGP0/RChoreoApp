use std::time::Duration;

use crate::choreography_settings;

use choreo_components::behavior::Behavior;
use choreo_components::choreography_settings::UpdateSelectedSceneBehavior;
use choreo_components::choreography_settings::UpdateSelectedSceneCommand;
use crossbeam_channel::unbounded;
use choreography_settings::Report;

#[test]
#[serial_test::serial]
fn update_selected_scene_behavior_spec() {
    let suite = rspec::describe("update selected scene behavior", (), |spec| {
        spec.it("syncs selected scene to view model and updates scene name", |_| {
            let context = choreography_settings::ChoreographySettingsTestContext::new();
            let (sender, receiver) = unbounded::<UpdateSelectedSceneCommand>();
            let behavior = UpdateSelectedSceneBehavior::new_with_receiver(
                context.global_state_store.clone(),
                receiver,
            );
            context.activate_behaviors(vec![Box::new(behavior) as Box<dyn Behavior<_>>]);

            let scene_id = choreo_master_mobile_json::SceneId(10);
            context.update_global_state(|state| {
                state.selected_scene = Some(choreo_components::scenes::SceneViewModel::new(
                    scene_id,
                    "Original",
                    choreo_master_mobile_json::Color::transparent(),
                ));
                state.choreography.scenes = vec![choreo_models::SceneModel {
                    scene_id,
                    positions: Vec::new(),
                    name: "Original".to_string(),
                    text: None,
                    fixed_positions: false,
                    timestamp: Some("3".to_string()),
                    variation_depth: 0,
                    variations: Vec::new(),
                    current_variation: Vec::new(),
                    color: choreo_master_mobile_json::Color::transparent(),
                }];
            });

            sender
                .send(UpdateSelectedSceneCommand::SyncFromSelected)
                .expect("send should succeed");

            let synced = context.wait_until(Duration::from_secs(1), || {
                context.view_model.borrow().scene_name == "Original"
            });
            assert!(synced);

            sender
                .send(UpdateSelectedSceneCommand::SceneName("Updated".to_string()))
                .expect("send should succeed");

            let renamed = context.wait_until(Duration::from_secs(1), || {
                context.read_global_state(|state| {
                    state
                        .selected_scene
                        .as_ref()
                        .map(|scene| scene.name.as_str())
                        == Some("Updated")
                        && state.choreography.scenes[0].name == "Updated"
                })
            });
            assert!(renamed);
        });
    });

    let report = choreography_settings::run_suite(&suite);
    assert!(report.is_success());
}
