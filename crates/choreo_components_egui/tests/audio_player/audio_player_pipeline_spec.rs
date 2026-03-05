use std::fs;
use std::rc::Rc;
use std::sync::mpsc::channel;
use std::time::SystemTime;

use choreo_master_mobile_json::Color;
use choreo_master_mobile_json::SceneId;
use choreo_models::ChoreographyModel;
use choreo_models::SceneModel;
use choreo_models::SettingsPreferenceKeys;

use choreo_components_egui::audio_player::AudioPlayerBackend;
use choreo_components_egui::audio_player::OpenAudioFileCommand;
use choreo_components_egui::audio_player::audio_player_behaviors::AudioPlayerBehaviorDependencies;
use choreo_components_egui::audio_player::build_audio_player_behaviors;
use choreo_components_egui::audio_player::messages::CloseAudioFileCommand;
use choreo_components_egui::audio_player::messages::LinkSceneToPositionCommand;
use choreo_components_egui::audio_player::runtime::AudioPlayerRuntime;
use choreo_components_egui::audio_player::state::AudioPlayerState;
use choreo_components_egui::global::GlobalStateActor;
use choreo_components_egui::global::SceneViewModel;
use choreo_components_egui::preferences::InMemoryPreferences;
use choreo_components_egui::preferences::Preferences;

#[test]
fn build_pipeline_handles_open_and_close_commands() {
    let (open_tx, open_rx) = channel();
    let (close_tx, close_rx) = channel();
    let (link_tx, link_rx) = channel();
    let (position_tx, _position_rx) = channel();

    let preferences = Rc::new(InMemoryPreferences::new());
    preferences.set_string(
        SettingsPreferenceKeys::AUDIO_PLAYER_BACKEND,
        AudioPlayerBackend::Rodio.as_preference().to_string(),
    );
    let global_state = GlobalStateActor::new();

    let pipeline = build_audio_player_behaviors(AudioPlayerBehaviorDependencies {
        global_state_store: Rc::clone(&global_state),
        open_audio_receiver: open_rx,
        close_audio_receiver: close_rx,
        position_changed_senders: vec![position_tx],
        link_scene_receiver: link_rx,
        preferences: preferences.clone(),
        haptic_feedback: None,
    });

    let file_path = create_temp_audio_file_path();
    let mut state = AudioPlayerState::default();
    let mut runtime = AudioPlayerRuntime::new(AudioPlayerBackend::Awedio);

    open_tx
        .send(OpenAudioFileCommand {
            file_path: file_path.clone(),
            trace_context: None,
        })
        .expect("open command should send");
    pipeline.open_audio_file.poll(&mut state, &mut runtime);

    assert!(state.has_player);
    assert!(state.has_stream_factory);
    assert_eq!(
        state.last_opened_audio_file_path.as_deref(),
        Some(file_path.as_str())
    );
    assert_eq!(
        preferences
            .get_string(SettingsPreferenceKeys::LAST_OPENED_AUDIO_FILE, "")
            .as_str(),
        file_path
    );

    close_tx
        .send(CloseAudioFileCommand {
            trace_context: None,
        })
        .expect("close command should send");
    pipeline.close_audio_file.poll(&mut state, &mut runtime);

    assert!(!state.has_player);
    assert!(!runtime.has_player());

    let _ = fs::remove_file(file_path);
    let _ = link_tx;
}

#[test]
fn pipeline_links_scene_and_publishes_position_changed_event() {
    let (open_tx, open_rx) = channel();
    let (close_tx, close_rx) = channel();
    let (link_tx, link_rx) = channel();
    let (position_tx, position_rx) = channel();

    let preferences = Rc::new(InMemoryPreferences::new());
    let global_state = GlobalStateActor::new();
    assert!(global_state.try_update(|state| {
        state.scenes = vec![
            scene_view_model(1, "A", Some(1.0)),
            scene_view_model(2, "B", None),
            scene_view_model(3, "C", Some(5.0)),
        ];
        state.selected_scene = state.scenes.get(1).cloned();
        state.choreography = ChoreographyModel {
            scenes: vec![
                scene_model(1, "A", Some("1.0")),
                scene_model(2, "B", None),
                scene_model(3, "C", Some("5.0")),
            ],
            ..ChoreographyModel::default()
        };
    }));

    let pipeline = build_audio_player_behaviors(AudioPlayerBehaviorDependencies {
        global_state_store: Rc::clone(&global_state),
        open_audio_receiver: open_rx,
        close_audio_receiver: close_rx,
        position_changed_senders: vec![position_tx],
        link_scene_receiver: link_rx,
        preferences,
        haptic_feedback: None,
    });

    let mut state = AudioPlayerState {
        position: 2.1,
        ..AudioPlayerState::default()
    };
    let mut runtime = AudioPlayerRuntime::new(AudioPlayerBackend::Rodio);
    pipeline.ticks.poll(&mut state, &mut runtime);

    link_tx
        .send(LinkSceneToPositionCommand {
            trace_context: None,
        })
        .expect("link command should send");
    pipeline.link_scene.poll(&mut state, None);

    assert_eq!(state.scenes[1].timestamp, Some(2.1));
    assert_eq!(
        state.choreography_scenes[1].timestamp.as_deref(),
        Some("2.1")
    );

    state.position = 3.0;
    pipeline.position_changed.poll(&mut state);
    let event = position_rx.recv().expect("position event should be sent");
    assert_eq!(event.position_seconds, 3.0);
    assert!(event.trace_context.is_none());

    let _ = open_tx;
    let _ = close_tx;
}

fn scene_model(scene_id: i32, name: &str, timestamp: Option<&str>) -> SceneModel {
    SceneModel {
        scene_id: SceneId(scene_id),
        positions: Vec::new(),
        name: name.to_string(),
        text: None,
        fixed_positions: false,
        timestamp: timestamp.map(str::to_string),
        variation_depth: 0,
        variations: Vec::new(),
        current_variation: Vec::new(),
        color: Color::transparent(),
    }
}

fn scene_view_model(scene_id: i32, name: &str, timestamp: Option<f64>) -> SceneViewModel {
    let mut scene = SceneViewModel::new(SceneId(scene_id), name, Color::transparent());
    scene.timestamp = timestamp;
    scene
}

fn create_temp_audio_file_path() -> String {
    let unique = SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system time should be after unix epoch")
        .as_nanos();
    let path = std::env::temp_dir().join(format!("rchoreo-audio-pipeline-{unique}.mp3"));
    fs::write(&path, b"audio-test").expect("temp audio file should be writable");
    path.to_string_lossy().into_owned()
}
