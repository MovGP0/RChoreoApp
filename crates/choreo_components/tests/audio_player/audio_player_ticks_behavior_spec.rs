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

macro_rules! check {
    ($errors:expr, $condition:expr) => {
        if !$condition {
            $errors.push(format!("condition failed: {}", stringify!($condition)));
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
fn audio_player_ticks_updates_tick_values_and_can_link_from_scene_state() {
    let mut state = AudioPlayerState {
        duration: 10.0,
        position: 2.0,
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
                    timestamp: Some(4.0),
                },
            ],
            selected_scene_id: Some(2),
            choreography_scenes: vec![
                AudioPlayerChoreographyScene {
                    scene_id: 1,
                    timestamp: Some("1.0".to_string()),
                },
                AudioPlayerChoreographyScene {
                    scene_id: 2,
                    timestamp: Some("4.0".to_string()),
                },
            ],
        },
    );
    reduce(&mut state, AudioPlayerAction::UpdateTicksAndLinkState);

    let mut errors = Vec::new();

    check_eq!(errors, state.tick_values, vec![1.0, 4.0]);
    check!(errors, state.can_link_scene_to_position);

    assert_no_errors(errors);
}
