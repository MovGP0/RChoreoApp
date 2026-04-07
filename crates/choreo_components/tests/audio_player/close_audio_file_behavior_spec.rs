use choreo_components::audio_player::actions::AudioPlayerAction;
use choreo_components::audio_player::reducer::reduce;
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

    let mut errors = Vec::new();

    check_eq!(errors, state.has_player, false);
    check_eq!(errors, state.has_stream_factory, false);
    check_eq!(errors, state.position, 0.0);
    check_eq!(errors, state.duration, 0.0);
    check_eq!(errors, state.is_playing, false);
    check_eq!(errors, state.can_seek, false);
    check_eq!(errors, state.can_set_speed, false);

    assert_no_errors(errors);
}
