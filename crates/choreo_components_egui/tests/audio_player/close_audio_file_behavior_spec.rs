use crate::audio_player::audio_player_component::actions::AudioPlayerAction;
use crate::audio_player::audio_player_component::reducer::reduce;
use crate::audio_player::audio_player_component::state::AudioPlayerState;

#[test]
fn close_audio_file_resets_player_state() {
    let mut state = AudioPlayerState {
        has_player: true,
        has_stream_factory: true,
        position: 5.0,
        duration: 40.0,
        is_playing: true,
        can_seek: true,
        can_set_speed: true,
        ..AudioPlayerState::default()
    };

    reduce(&mut state, AudioPlayerAction::CloseAudioFile);

    assert!(!state.has_player);
    assert!(!state.has_stream_factory);
    assert_eq!(state.position, 0.0);
    assert_eq!(state.duration, 0.0);
    assert!(!state.is_playing);
    assert!(!state.can_seek);
    assert!(!state.can_set_speed);
}
