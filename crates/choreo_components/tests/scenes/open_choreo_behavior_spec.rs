use std::fs;
use std::path::PathBuf;
use std::rc::Rc;
use std::time::Duration;

use crate::scenes;

use choreo_components::audio_player::AudioPlayerPositionChangedEvent;
use choreo_components::audio_player::CloseAudioFileCommand;
use choreo_components::audio_player::OpenAudioFileCommand;
use choreo_components::preferences::Preferences;
use choreo_components::scenes::OpenChoreoActions;
use choreo_components::scenes::OpenChoreoRequested;
use choreo_components::scenes::ScenesDependencies;
use choreo_components::scenes::ScenesProvider;
use choreo_models::SettingsPreferenceKeys;
use crossbeam_channel::unbounded;
use scenes::Report;

fn unique_temp_dir(name: &str) -> PathBuf {
    let mut path = std::env::temp_dir();
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("current time should be after unix epoch")
        .as_nanos();
    path.push(format!("rchoreo-{name}-{nanos}"));
    fs::create_dir_all(&path).expect("temp dir should be created");
    path
}

fn build_choreo_json(name: &str, show_timestamps: bool, audio_path: Option<String>) -> String {
    let mut model = choreo_master_mobile_json::Choreography {
        name: name.to_string(),
        ..Default::default()
    };
    model.settings.show_timestamps = show_timestamps;
    model.settings.music_path_absolute = audio_path;
    model.scenes = vec![choreo_master_mobile_json::Scene {
        name: "Scene 1".to_string(),
        timestamp: Some("00:03".to_string()),
        ..Default::default()
    }];

    choreo_master_mobile_json::export(&model).expect("json export should succeed")
}

#[test]
#[serial_test::serial]
fn open_choreo_behavior_spec() {
    let suite = rspec::describe("open choreo behavior", (), |spec| {
        spec.it(
            "loads choreography from contents and triggers scene/audio updates",
            |_| {
                let context = scenes::ScenesTestContext::new();
                let temp_dir = unique_temp_dir("open-choreo");
                let choreo_path = temp_dir.join("dance.choreo");
                let audio_path = temp_dir.join("music.mp3");
                fs::write(&audio_path, b"audio").expect("audio file should be created");
                let json = build_choreo_json(
                    "Imported",
                    true,
                    Some(audio_path.to_string_lossy().into_owned()),
                );

                let (open_audio_sender, open_audio_receiver) = unbounded::<OpenAudioFileCommand>();
                let (close_audio_sender, close_audio_receiver) =
                    unbounded::<CloseAudioFileCommand>();
                let (audio_position_sender, audio_position_receiver) =
                    unbounded::<AudioPlayerPositionChangedEvent>();
                let (show_dialog_sender, _show_dialog_receiver) = unbounded();
                let (close_dialog_sender, _close_dialog_receiver) = unbounded();
                let (show_timestamps_sender, show_timestamps_receiver) = unbounded();
                let (redraw_floor_sender, _redraw_floor_receiver) = unbounded();
                let preferences_dyn = context.preferences.clone()
                    as Rc<dyn choreo_components::preferences::Preferences>;
                let provider = ScenesProvider::new(ScenesDependencies {
                    global_state: context.global_state_store.state_handle(),
                    global_state_store: context.global_state_store.clone(),
                    state_machine: None,
                    preferences: preferences_dyn,
                    show_dialog_sender,
                    close_dialog_sender,
                    haptic_feedback: None,
                    open_audio_sender,
                    close_audio_sender,
                    show_timestamps_sender,
                    show_timestamps_receiver: show_timestamps_receiver.clone(),
                    redraw_floor_sender,
                    audio_position_receiver,
                    actions: OpenChoreoActions::default(),
                });
                let open_choreo_sender = provider.open_choreo_sender();
                let _scenes_view_model = provider.scenes_view_model();
                drop(audio_position_sender);

                open_choreo_sender
                    .send(OpenChoreoRequested {
                        file_path: Some(choreo_path.to_string_lossy().into_owned()),
                        file_name: None,
                        contents: json,
                    })
                    .expect("open request should be sent");

                let loaded = context.wait_until(Duration::from_secs(1), || {
                    context.read_global_state(|state| state.choreography.name == "Imported")
                });
                assert!(loaded);

                assert!(
                    context.read_global_state(|state| state.choreography.settings.show_timestamps)
                );

                let open_audio = open_audio_receiver
                    .try_recv()
                    .expect("open audio command should be emitted");
                assert_eq!(open_audio.file_path, audio_path.to_string_lossy());
                assert!(close_audio_receiver.try_recv().is_err());

                let last_opened = context
                    .preferences
                    .get_string(SettingsPreferenceKeys::LAST_OPENED_CHOREO_FILE, "");
                assert_eq!(last_opened, choreo_path.to_string_lossy());

                fs::remove_dir_all(temp_dir).expect("temp dir should be removed");
            },
        );

        spec.it(
            "closes audio and stores file name when importing without file path",
            |_| {
                let context = scenes::ScenesTestContext::new();
                let json = build_choreo_json("FromFileName", false, None);

                let (open_audio_sender, open_audio_receiver) = unbounded::<OpenAudioFileCommand>();
                let (close_audio_sender, close_audio_receiver) =
                    unbounded::<CloseAudioFileCommand>();
                let (audio_position_sender, audio_position_receiver) =
                    unbounded::<AudioPlayerPositionChangedEvent>();
                let (show_dialog_sender, _show_dialog_receiver) = unbounded();
                let (close_dialog_sender, _close_dialog_receiver) = unbounded();
                let (show_timestamps_sender, show_timestamps_receiver) = unbounded();
                let (redraw_floor_sender, _redraw_floor_receiver) = unbounded();
                let preferences_dyn = context.preferences.clone()
                    as Rc<dyn choreo_components::preferences::Preferences>;
                let provider = ScenesProvider::new(ScenesDependencies {
                    global_state: context.global_state_store.state_handle(),
                    global_state_store: context.global_state_store.clone(),
                    state_machine: None,
                    preferences: preferences_dyn,
                    show_dialog_sender,
                    close_dialog_sender,
                    haptic_feedback: None,
                    open_audio_sender,
                    close_audio_sender,
                    show_timestamps_sender,
                    show_timestamps_receiver: show_timestamps_receiver.clone(),
                    redraw_floor_sender,
                    audio_position_receiver,
                    actions: OpenChoreoActions::default(),
                });
                let open_choreo_sender = provider.open_choreo_sender();
                let _scenes_view_model = provider.scenes_view_model();
                drop(audio_position_sender);

                open_choreo_sender
                    .send(OpenChoreoRequested {
                        file_path: None,
                        file_name: Some("browser-import.choreo".to_string()),
                        contents: json,
                    })
                    .expect("open request should be sent");

                let loaded = context.wait_until(Duration::from_secs(1), || {
                    context.read_global_state(|state| state.choreography.name == "FromFileName")
                });
                assert!(loaded);

                assert!(close_audio_receiver.try_recv().is_ok());
                assert!(open_audio_receiver.try_recv().is_err());
                assert_eq!(
                    context
                        .preferences
                        .get_string(SettingsPreferenceKeys::LAST_OPENED_CHOREO_FILE, ""),
                    "browser-import.choreo"
                );
            },
        );
    });

    let report = scenes::run_suite(&suite);
    assert!(report.is_success());
}
