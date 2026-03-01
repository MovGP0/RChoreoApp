use crate::choreo_main::Report;
use crate::choreo_main::actions::ChoreoMainAction;
use crate::choreo_main::reducer::reduce;
use crate::choreo_main::state::ChoreoMainState;
use crate::choreo_main::state::SceneState;

#[test]
fn timestamp_sync_spec() {
    let suite = rspec::describe("timestamp synchronization", (), |spec| {
        spec.it(
            "updates audio position when selecting a timestamped scene",
            |_| {
                let mut state = ChoreoMainState::default();
                reduce(
                    &mut state,
                    ChoreoMainAction::SetScenes {
                        scenes: vec![
                            SceneState {
                                name: "Scene 1".to_string(),
                                timestamp_seconds: Some(5.0),
                            },
                            SceneState {
                                name: "Scene 2".to_string(),
                                timestamp_seconds: Some(10.0),
                            },
                        ],
                    },
                );

                reduce(&mut state, ChoreoMainAction::SelectScene { index: 1 });

                assert_eq!(state.floor_scene_name.as_deref(), Some("Scene 2"));
                assert_eq!(state.selected_scene_index, Some(1));
                assert!((state.audio_position_seconds - 10.0).abs() < f64::EPSILON);
            },
        );

        spec.it(
            "keeps audio position when selecting a scene without timestamp",
            |_| {
                let mut state = ChoreoMainState::default();
                reduce(
                    &mut state,
                    ChoreoMainAction::SetScenes {
                        scenes: vec![
                            SceneState {
                                name: "Scene 1".to_string(),
                                timestamp_seconds: Some(5.0),
                            },
                            SceneState {
                                name: "Scene 2".to_string(),
                                timestamp_seconds: None,
                            },
                        ],
                    },
                );
                assert!((state.audio_position_seconds - 5.0).abs() < f64::EPSILON);

                reduce(&mut state, ChoreoMainAction::SelectScene { index: 1 });

                assert_eq!(state.floor_scene_name.as_deref(), Some("Scene 2"));
                assert_eq!(state.selected_scene_index, Some(1));
                assert!((state.audio_position_seconds - 5.0).abs() < f64::EPSILON);
            },
        );

        spec.it(
            "updates selected scene when audio position moves into a later scene range",
            |_| {
                let mut state = ChoreoMainState::default();
                reduce(
                    &mut state,
                    ChoreoMainAction::SetScenes {
                        scenes: vec![
                            SceneState {
                                name: "Scene 1".to_string(),
                                timestamp_seconds: Some(5.0),
                            },
                            SceneState {
                                name: "Scene 2".to_string(),
                                timestamp_seconds: Some(10.0),
                            },
                            SceneState {
                                name: "Scene 3".to_string(),
                                timestamp_seconds: Some(20.0),
                            },
                        ],
                    },
                );
                reduce(
                    &mut state,
                    ChoreoMainAction::UpdateAudioPosition { seconds: 12.0 },
                );

                assert_eq!(state.floor_scene_name.as_deref(), Some("Scene 2"));
                assert_eq!(state.selected_scene_index, Some(1));

                reduce(&mut state, ChoreoMainAction::SelectScene { index: 2 });
                assert_eq!(state.floor_scene_name.as_deref(), Some("Scene 3"));
            },
        );
    });

    let report = crate::choreo_main::run_suite(&suite);
    assert!(report.is_success());
}
