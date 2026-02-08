use std::time::Duration;

use crate::audio_player;

use audio_player::Report;
use choreo_components::audio_player::AudioPlayerLinkSceneBehavior;
use choreo_components::audio_player::LinkSceneToPositionCommand;
use choreo_components::behavior::Behavior;
use crossbeam_channel::unbounded;

#[test]
#[serial_test::serial]
fn audio_player_link_scene_behavior_spec() {
    let suite = rspec::describe("audio player link scene behavior", (), |spec| {
        spec.it("links selected scene timestamp when position is between neighbors", |_| {
            let (link_sender, link_receiver) = unbounded::<LinkSceneToPositionCommand>();
            let global_state = choreo_components::global::GlobalStateActor::new();
            let behavior = AudioPlayerLinkSceneBehavior::new(
                global_state.clone(),
                link_receiver,
            );
            let context = audio_player::AudioPlayerTestContext::with_global_state(vec![
                Box::new(behavior) as Box<dyn Behavior<_>>,
            ], global_state);

            let first = audio_player::scene_view_model(1, "A", Some("00:01"));
            let second = audio_player::scene_view_model(2, "B", Some("00:03"));
            let third = audio_player::scene_view_model(3, "C", Some("00:09"));
            context.update_global_state(|state| {
                state.scenes = vec![first.clone(), second.clone(), third.clone()];
                state.selected_scene = Some(second.clone());
                state.choreography.scenes = vec![
                    choreo_models::SceneModel {
                        scene_id: first.scene_id,
                        positions: Vec::new(),
                        name: first.name.clone(),
                        text: None,
                        fixed_positions: false,
                        timestamp: Some("1".to_string()),
                        variation_depth: 0,
                        variations: Vec::new(),
                        current_variation: Vec::new(),
                        color: first.color.clone(),
                    },
                    choreo_models::SceneModel {
                        scene_id: second.scene_id,
                        positions: Vec::new(),
                        name: second.name.clone(),
                        text: None,
                        fixed_positions: false,
                        timestamp: Some("3".to_string()),
                        variation_depth: 0,
                        variations: Vec::new(),
                        current_variation: Vec::new(),
                        color: second.color.clone(),
                    },
                    choreo_models::SceneModel {
                        scene_id: third.scene_id,
                        positions: Vec::new(),
                        name: third.name.clone(),
                        text: None,
                        fixed_positions: false,
                        timestamp: Some("9".to_string()),
                        variation_depth: 0,
                        variations: Vec::new(),
                        current_variation: Vec::new(),
                        color: third.color.clone(),
                    },
                ];
            });

            context.view_model.borrow_mut().position = 5.24;
            link_sender
                .send(LinkSceneToPositionCommand)
                .expect("send should succeed");

            let linked = context.wait_until(Duration::from_secs(1), || {
                context.read_global_state(|state| {
                    state
                        .selected_scene
                        .as_ref()
                        .and_then(|scene| scene.timestamp)
                        .map(|value| (value - 5.2).abs() < 0.0001)
                        .unwrap_or(false)
                })
            });
            assert!(linked);

            let model_timestamp = context.read_global_state(|state| {
                state.choreography.scenes[1].timestamp.clone()
            });
            assert_eq!(model_timestamp.as_deref(), Some("5.2"));
        });
    });

    let report = audio_player::run_suite(&suite);
    assert!(report.is_success());
}
