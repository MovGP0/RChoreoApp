use crate::audio_player::audio_player_component::actions::AudioPlayerAction;
use crate::audio_player::audio_player_component::reducer::AudioPlayerEffect;
use crate::audio_player::audio_player_component::reducer::reduce;
use crate::audio_player::audio_player_component::state::AudioPlayerState;

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
