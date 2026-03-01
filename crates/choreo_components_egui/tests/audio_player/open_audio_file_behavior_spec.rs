use crate::audio_player::audio_player_component::actions::AudioPlayerAction;
use crate::audio_player::audio_player_component::reducer::reduce;
use crate::audio_player::audio_player_component::state::AudioPlayerState;

#[test]
fn open_audio_file_sets_stream_factory_flag_and_persists_last_opened_path() {
    let mut state = AudioPlayerState::default();

    reduce(
        &mut state,
        AudioPlayerAction::OpenAudioFile {
            file_path: "C:\\temp\\audio.mp3".to_string(),
            file_exists: true,
        },
    );

    assert!(state.has_stream_factory);
    assert!(state.has_player);
    assert_eq!(
        state.last_opened_audio_file_path.as_deref(),
        Some("C:\\temp\\audio.mp3")
    );
}

#[test]
fn open_audio_file_ignores_empty_paths() {
    let mut state = AudioPlayerState::default();

    reduce(
        &mut state,
        AudioPlayerAction::OpenAudioFile {
            file_path: "   ".to_string(),
            file_exists: true,
        },
    );

    assert!(!state.has_stream_factory);
    assert!(state.last_opened_audio_file_path.is_none());
}

#[test]
fn open_audio_file_keeps_player_disabled_when_file_does_not_exist() {
    let mut state = AudioPlayerState {
        can_seek: true,
        can_set_speed: true,
        duration: 40.0,
        position: 5.0,
        is_playing: true,
        ..AudioPlayerState::default()
    };

    reduce(
        &mut state,
        AudioPlayerAction::OpenAudioFile {
            file_path: "C:\\temp\\missing.mp3".to_string(),
            file_exists: false,
        },
    );

    assert!(state.has_stream_factory);
    assert!(!state.has_player);
    assert!(!state.can_seek);
    assert!(!state.can_set_speed);
    assert_eq!(state.duration, 0.0);
    assert_eq!(state.position, 0.0);
    assert!(!state.is_playing);
    assert_eq!(
        state.last_opened_audio_file_path.as_deref(),
        Some("C:\\temp\\missing.mp3")
    );
}
