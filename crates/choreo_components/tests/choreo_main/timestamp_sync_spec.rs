use crate::choreo_main::Report;
use crate::choreo_main::actions::ChoreoMainAction;
use crate::choreo_main::reducer::reduce;
use crate::choreo_main::state::ChoreoMainState;
use crate::choreo_main::state::SceneState;

macro_rules! check_eq {
    ($errors:expr, $left:expr, $right:expr) => {
        if $left != $right {
            $errors.push(format!(
                "{} != {} (left = {:?}, right = {:?})",
                stringify!($left),
                stringify!($right),
                $left,
                $right
            ));
        }
    };
}

macro_rules! check_close {
    ($errors:expr, $left:expr, $right:expr) => {
        if ($left - $right).abs() >= f64::EPSILON {
            $errors.push(format!(
                "{} ~= {} failed (left = {:?}, right = {:?})",
                stringify!($left),
                stringify!($right),
                $left,
                $right
            ));
        }
    };
}

fn assert_no_errors(errors: Vec<String>) {
    assert!(
        errors.is_empty(),
        "Assertion failures:\n{}",
        errors.join("\n")
    );
}

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

                let mut errors = Vec::new();

                check_eq!(errors, state.floor_scene_name.as_deref(), Some("Scene 2"));
                check_eq!(errors, state.selected_scene_index, Some(1));
                check_close!(errors, state.audio_position_seconds, 10.0);

                assert_no_errors(errors);
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

                let mut errors = Vec::new();

                check_close!(errors, state.audio_position_seconds, 5.0);

                reduce(&mut state, ChoreoMainAction::SelectScene { index: 1 });

                check_eq!(errors, state.floor_scene_name.as_deref(), Some("Scene 2"));
                check_eq!(errors, state.selected_scene_index, Some(1));
                check_close!(errors, state.audio_position_seconds, 5.0);

                assert_no_errors(errors);
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

                let mut errors = Vec::new();

                check_eq!(errors, state.floor_scene_name.as_deref(), Some("Scene 2"));
                check_eq!(errors, state.selected_scene_index, Some(1));
                check_close!(errors, state.audio_position_seconds, 12.0);

                reduce(&mut state, ChoreoMainAction::SelectScene { index: 2 });

                check_eq!(errors, state.floor_scene_name.as_deref(), Some("Scene 3"));
                check_eq!(errors, state.selected_scene_index, Some(2));
                check_close!(errors, state.audio_position_seconds, 12.0);

                assert_no_errors(errors);
            },
        );

        spec.it(
            "links selected scene timestamp to current audio position rounded to 100ms",
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
                reduce(
                    &mut state,
                    ChoreoMainAction::UpdateAudioPosition { seconds: 12.345 },
                );

                reduce(
                    &mut state,
                    ChoreoMainAction::LinkSelectedSceneToAudioPosition,
                );

                let mut errors = Vec::new();

                check_eq!(errors, state.floor_scene_name.as_deref(), Some("Scene 2"));
                check_eq!(errors, state.selected_scene_index, Some(1));
                check_close!(errors, state.audio_position_seconds, 12.3);
                check_eq!(
                    errors,
                    state.scenes[1].timestamp_seconds,
                    Some(state.audio_position_seconds)
                );

                assert_no_errors(errors);
            },
        );
    });

    let report = crate::choreo_main::run_suite(&suite);
    assert!(report.is_success());
}
