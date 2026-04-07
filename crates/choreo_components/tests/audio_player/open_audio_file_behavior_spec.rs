use std::rc::Rc;
use std::sync::mpsc::channel;

use choreo_models::SettingsPreferenceKeys;

use choreo_components::audio_player::AudioPlayerBackend;
use choreo_components::audio_player::AudioPlayerPipelineDependencies;
use choreo_components::audio_player::OpenAudioFileCommand;
use choreo_components::audio_player::actions::AudioPlayerAction;
use choreo_components::audio_player::build_audio_player_pipeline;
use choreo_components::audio_player::reducer::reduce;
use choreo_components::audio_player::runtime::AudioPlayerRuntime;
use choreo_components::audio_player::state::AudioPlayerState;
use choreo_components::global::GlobalStateActor;
use choreo_components::preferences::InMemoryPreferences;
use choreo_components::preferences::Preferences;

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
fn open_audio_file_sets_stream_factory_flag_and_persists_last_opened_path() {
    let mut state = AudioPlayerState::default();

    reduce(
        &mut state,
        AudioPlayerAction::OpenAudioFile {
            file_path: "C:\\temp\\audio.mp3".to_string(),
            file_exists: true,
        },
    );

    let mut errors = Vec::new();

    check!(errors, state.has_stream_factory);
    check!(errors, state.has_player);
    check_eq!(
        errors,
        state.last_opened_audio_file_path.as_deref(),
        Some("C:\\temp\\audio.mp3")
    );

    assert_no_errors(errors);
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

    let mut errors = Vec::new();

    check!(errors, !state.has_stream_factory);
    check!(errors, state.last_opened_audio_file_path.is_none());

    assert_no_errors(errors);
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

    let mut errors = Vec::new();

    check!(errors, state.has_stream_factory);
    check!(errors, !state.has_player);
    check!(errors, !state.can_seek);
    check!(errors, !state.can_set_speed);
    check_eq!(errors, state.duration, 0.0);
    check_eq!(errors, state.position, 0.0);
    check!(errors, !state.is_playing);
    check_eq!(
        errors,
        state.last_opened_audio_file_path.as_deref(),
        Some("C:\\temp\\missing.mp3")
    );

    assert_no_errors(errors);
}

#[test]
fn open_behavior_reads_backend_and_persists_last_opened_path_for_missing_file() {
    let (open_tx, open_rx) = channel();
    let (_close_tx, close_rx) = channel();
    let (_link_tx, link_rx) = channel();
    let (position_tx, _position_rx) = channel();

    let preferences = Rc::new(InMemoryPreferences::new());
    preferences.set_string(
        SettingsPreferenceKeys::AUDIO_PLAYER_BACKEND,
        AudioPlayerBackend::Awedio.as_preference().to_string(),
    );

    let pipeline = build_audio_player_pipeline(AudioPlayerPipelineDependencies {
        global_state_store: GlobalStateActor::new(),
        open_audio_receiver: open_rx,
        close_audio_receiver: close_rx,
        position_changed_senders: vec![position_tx],
        link_scene_receiver: link_rx,
        preferences: preferences.clone(),
        haptic_feedback: None,
    });

    let file_path = "C:\\temp\\missing-parity.mp3".to_string();
    open_tx
        .send(OpenAudioFileCommand {
            file_path: file_path.clone(),
            trace_context: None,
        })
        .expect("open command should send");

    let mut state = AudioPlayerState::default();
    let mut runtime = AudioPlayerRuntime::new(AudioPlayerBackend::Rodio);
    pipeline.open_audio_file.poll(&mut state, &mut runtime);

    let mut errors = Vec::new();

    check!(errors, state.has_stream_factory);
    check!(errors, !state.has_player);
    check_eq!(
        errors,
        state.last_opened_audio_file_path,
        Some(file_path.clone())
    );
    check_eq!(
        errors,
        preferences.get_string(SettingsPreferenceKeys::LAST_OPENED_AUDIO_FILE, ""),
        file_path
    );

    assert_no_errors(errors);
}
