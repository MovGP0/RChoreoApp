use crate::audio_player::audio_player_component::actions::AudioPlayerAction;
use crate::audio_player::audio_player_component::reducer::reduce;
use crate::audio_player::audio_player_component::state::AudioPlayerChoreographyScene;
use crate::audio_player::audio_player_component::state::AudioPlayerScene;
use crate::audio_player::audio_player_component::state::AudioPlayerState;

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

    let selected = state
        .scenes
        .iter()
        .find(|scene| scene.scene_id == 2)
        .expect("selected scene should exist");
    assert_eq!(selected.timestamp, Some(5.2));

    let selected_model = state
        .choreography_scenes
        .iter()
        .find(|scene| scene.scene_id == 2)
        .expect("selected model scene should exist");
    assert_eq!(selected_model.timestamp.as_deref(), Some("5.2"));
}
