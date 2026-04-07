use choreo_components::audio_player::actions::AudioPlayerAction;
use choreo_components::audio_player::reducer::reduce;
use choreo_components::audio_player::state::AudioPlayerChoreographyScene;
use choreo_components::audio_player::state::AudioPlayerScene;
use choreo_components::audio_player::state::AudioPlayerState;

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

fn assert_no_errors(errors: Vec<String>) {
    assert!(
        errors.is_empty(),
        "Assertion failures:\n{}",
        errors.join("\n")
    );
}

#[test]
fn audio_player_link_scene_updates_selected_scene_timestamp_with_neighbor_bounds() {
    let mut state = AudioPlayerState {
        duration: 10.0,
        position: 5.24,
        ..AudioPlayerState::default()
    };

    reduce(
        &mut state,
        AudioPlayerAction::SetScenes {
            scenes: vec![
                AudioPlayerScene {
                    scene_id: 1,
                    name: "A".to_string(),
                    timestamp: Some(1.0),
                },
                AudioPlayerScene {
                    scene_id: 2,
                    name: "B".to_string(),
                    timestamp: Some(3.0),
                },
                AudioPlayerScene {
                    scene_id: 3,
                    name: "C".to_string(),
                    timestamp: Some(9.0),
                },
            ],
            selected_scene_id: Some(2),
            choreography_scenes: vec![
                AudioPlayerChoreographyScene {
                    scene_id: 1,
                    timestamp: Some("1".to_string()),
                },
                AudioPlayerChoreographyScene {
                    scene_id: 2,
                    timestamp: Some("3".to_string()),
                },
                AudioPlayerChoreographyScene {
                    scene_id: 3,
                    timestamp: Some("9".to_string()),
                },
            ],
        },
    );
    reduce(&mut state, AudioPlayerAction::UpdateTicksAndLinkState);
    reduce(&mut state, AudioPlayerAction::LinkSceneToPosition);

    let mut errors = Vec::new();

    let selected = state
        .scenes
        .iter()
        .find(|scene| scene.scene_id == 2)
        .expect("selected scene should exist");
    check_eq!(errors, selected.timestamp, Some(5.2));

    let selected_model = state
        .choreography_scenes
        .iter()
        .find(|scene| scene.scene_id == 2)
        .expect("selected model scene should exist");

    check_eq!(errors, selected_model.timestamp.as_deref(), Some("5.2"));

    assert_no_errors(errors);
}
