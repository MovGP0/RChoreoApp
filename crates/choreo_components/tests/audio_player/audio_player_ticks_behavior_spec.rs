use choreo_components::audio_player::actions::AudioPlayerAction;
use choreo_components::audio_player::reducer::reduce;
use choreo_components::audio_player::state::AudioPlayerChoreographyScene;
use choreo_components::audio_player::state::AudioPlayerScene;
use choreo_components::audio_player::state::AudioPlayerState;

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

    assert_eq!(state.tick_values, vec![1.0, 4.0]);
    assert!(state.can_link_scene_to_position);
}
