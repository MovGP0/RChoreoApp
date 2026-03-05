use choreo_components_egui::audio_player::actions::AudioPlayerAction;
use choreo_components_egui::audio_player::reducer::AudioPlayerEffect;
use choreo_components_egui::audio_player::reducer::reduce;
use choreo_components_egui::audio_player::state::AudioPlayerState;

#[test]
fn audio_player_position_changed_emits_events_only_when_position_changes() {
    let mut state = AudioPlayerState {
        position: 1.0,
        ..AudioPlayerState::default()
    };

    let effects = reduce(&mut state, AudioPlayerAction::PublishPositionIfChanged);
    assert_eq!(
        effects,
        vec![AudioPlayerEffect::PositionChangedPublished {
            position_seconds: 1.0
        }]
    );

    let no_effect = reduce(&mut state, AudioPlayerAction::PublishPositionIfChanged);
    assert!(no_effect.is_empty());

    state.position = 2.0;
    let effects_again = reduce(&mut state, AudioPlayerAction::PublishPositionIfChanged);
    assert_eq!(
        effects_again,
        vec![AudioPlayerEffect::PositionChangedPublished {
            position_seconds: 2.0
        }]
    );
}
