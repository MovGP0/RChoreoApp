use std::cell::RefCell;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::rc::Rc;
use std::thread;
use std::time::Duration;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use choreo_components::audio_player::OpenAudioFileCommand;
use choreo_components::audio_player::actions::AudioPlayerAction;
use choreo_components::choreo_main::ChoreoMainBehaviorDependencies;
use choreo_components::choreo_main::MainPageActionHandlers;
use choreo_components::choreo_main::MainPageBinding;
use choreo_components::choreo_main::MainPageDependencies;
use choreo_components::choreo_main::OpenAudioRequested;
use choreo_components::choreo_main::OpenImageRequested;
use choreo_components::choreo_main::ShowDialogCommand;
use choreo_components::choreo_main::actions::ChoreoMainAction;
use choreo_components::choreo_main::actions::OpenChoreoRequested;
use choreo_components::choreo_main::state::InteractionMode;
use choreo_components::choreography_settings::actions::ChoreographySettingsAction;
use choreo_components::choreography_settings::actions::UpdateSelectedSceneAction;
use choreo_components::observability::TraceContext;
use choreo_master_mobile_json::export;
use choreo_master_mobile_json::import;
use choreo_models::ChoreographyModel;
use choreo_models::ChoreographyModelMapper;
use crossbeam_channel::bounded;

#[test]
fn binding_forwards_open_audio_request_with_typed_trace_context() {
    let routed_requests: Rc<RefCell<Vec<OpenAudioRequested>>> = Rc::new(RefCell::new(Vec::new()));
    let routed_requests_for_handler = Rc::clone(&routed_requests);

    let binding = MainPageBinding::new(MainPageDependencies {
        action_handlers: MainPageActionHandlers {
            request_open_audio: Some(Rc::new(move |request| {
                routed_requests_for_handler.borrow_mut().push(request);
            })),
            ..MainPageActionHandlers::default()
        },
        ..MainPageDependencies::default()
    });

    let trace_context = TraceContext {
        trace_id_hex: Some("abc123".to_string()),
        span_id_hex: Some("def456".to_string()),
    };
    binding.request_open_audio(OpenAudioRequested {
        file_path: "C:/track.mp3".to_string(),
        trace_context: Some(trace_context.clone()),
    });

    let routed_requests = routed_requests.borrow();
    assert_eq!(routed_requests.len(), 1);
    assert_eq!(routed_requests[0].file_path, "C:/track.mp3");
    assert_eq!(routed_requests[0].trace_context, Some(trace_context));
}

#[test]
fn binding_routes_open_image_request_to_host_handler() {
    let routed_images: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(Vec::new()));
    let routed_images_for_handler = Rc::clone(&routed_images);
    let binding = MainPageBinding::new(MainPageDependencies {
        action_handlers: MainPageActionHandlers {
            request_open_image: Some(Rc::new(move |path| {
                routed_images_for_handler.borrow_mut().push(path);
            })),
            ..MainPageActionHandlers::default()
        },
        ..MainPageDependencies::default()
    });

    binding.request_open_image(OpenImageRequested {
        file_path: "C:/floor.svg".to_string(),
    });

    let routed_images = routed_images.borrow();
    assert_eq!(routed_images.as_slice(), ["C:/floor.svg"]);
}

#[test]
fn binding_updates_dialog_and_interaction_mode_state() {
    let binding = MainPageBinding::new(MainPageDependencies::default());

    binding.show_dialog(ShowDialogCommand {
        content: Some("Confirm".to_string()),
    });
    binding.dispatch(ChoreoMainAction::SelectMode { index: 5 });

    let state = binding.state();
    let state = state.borrow();
    assert!(state.is_dialog_open);
    assert_eq!(state.dialog_content.as_deref(), Some("Confirm"));
    assert_eq!(state.interaction_mode, InteractionMode::LineOfSight);
}

#[test]
fn binding_uses_pick_choreo_handler_to_load_selected_file() {
    let mut choreography = ChoreographyModel {
        name: "Loaded choreo".to_string(),
        ..ChoreographyModel::default()
    };
    choreography.scenes = vec![choreo_models::SceneModel {
        scene_id: choreo_master_mobile_json::SceneId(1),
        positions: Vec::new(),
        name: "Intro".to_string(),
        text: None,
        fixed_positions: false,
        timestamp: Some("00:01.500".to_string()),
        variation_depth: 0,
        variations: Vec::new(),
        current_variation: Vec::new(),
        color: choreo_master_mobile_json::Color::transparent(),
    }];
    let mapper = ChoreographyModelMapper;
    let contents = export(&mapper.map_to_json(&choreography))
        .expect("test choreography should serialize to json");

    let binding = MainPageBinding::new(MainPageDependencies {
        action_handlers: MainPageActionHandlers {
            pick_choreo_file: Some(Rc::new(move || {
                Some(OpenChoreoRequested {
                    file_path: Some("C:/demo.choreo".to_string()),
                    file_name: Some("demo.choreo".to_string()),
                    contents: contents.clone(),
                })
            })),
            ..MainPageActionHandlers::default()
        },
        ..MainPageDependencies::default()
    });

    binding.dispatch(ChoreoMainAction::RequestOpenChoreo(OpenChoreoRequested {
        file_path: None,
        file_name: None,
        contents: String::new(),
    }));

    let state = binding.state();
    let state = state.borrow();
    assert_eq!(state.scenes.len(), 1);
    assert_eq!(state.scenes[0].name, "Intro");
    assert_eq!(state.selected_scene_index, Some(0));
    assert_eq!(state.floor_scene_name.as_deref(), Some("Intro"));
    assert_eq!(state.choreography_settings_state.name, "Loaded choreo");
}

#[test]
fn binding_saves_current_choreography_back_to_last_opened_file() {
    let temp_file = unique_temp_file("choreo");
    let mut choreography = ChoreographyModel {
        name: "Before save".to_string(),
        ..ChoreographyModel::default()
    };
    choreography.scenes = vec![choreo_models::SceneModel {
        scene_id: choreo_master_mobile_json::SceneId(1),
        positions: Vec::new(),
        name: "Intro".to_string(),
        text: None,
        fixed_positions: false,
        timestamp: Some("1".to_string()),
        variation_depth: 0,
        variations: Vec::new(),
        current_variation: Vec::new(),
        color: choreo_master_mobile_json::Color::transparent(),
    }];
    let mapper = ChoreographyModelMapper;
    let contents = export(&mapper.map_to_json(&choreography))
        .expect("test choreography should serialize to json");
    fs::write(&temp_file, &contents).expect("temp .choreo file should be created");

    let file_path = temp_file.to_string_lossy().into_owned();
    let file_name = temp_file
        .file_name()
        .and_then(|value| value.to_str())
        .expect("temp file should have a name")
        .to_string();
    let contents_for_picker = contents.clone();
    let file_path_for_picker = file_path.clone();
    let file_name_for_picker = file_name.clone();

    let binding = MainPageBinding::new(MainPageDependencies {
        action_handlers: MainPageActionHandlers {
            pick_choreo_file: Some(Rc::new(move || {
                Some(OpenChoreoRequested {
                    file_path: Some(file_path_for_picker.clone()),
                    file_name: Some(file_name_for_picker.clone()),
                    contents: contents_for_picker.clone(),
                })
            })),
            ..MainPageActionHandlers::default()
        },
        ..MainPageDependencies::default()
    });

    binding.dispatch(ChoreoMainAction::RequestOpenChoreo(OpenChoreoRequested {
        file_path: None,
        file_name: None,
        contents: String::new(),
    }));
    binding.dispatch(ChoreoMainAction::ChoreographySettingsAction(
        ChoreographySettingsAction::UpdateName("Saved choreo".to_string()),
    ));
    binding.dispatch(ChoreoMainAction::ChoreographySettingsAction(
        ChoreographySettingsAction::UpdateSelectedScene(UpdateSelectedSceneAction::SceneName(
            "Saved intro".to_string(),
        )),
    ));
    binding.dispatch(ChoreoMainAction::RequestSaveChoreo);

    let saved_contents =
        fs::read_to_string(&temp_file).expect("saved .choreo file should be readable");
    let saved_json = import(&saved_contents).expect("saved .choreo contents should import");

    assert_eq!(saved_json.name, "Saved choreo");
    assert_eq!(saved_json.scenes.len(), 1);
    assert_eq!(saved_json.scenes[0].name, "Saved intro");

    let _ = fs::remove_file(temp_file);
}

#[test]
fn binding_uses_pick_audio_handler_to_open_selected_file_and_show_audio_panel() {
    let (open_audio_sender, open_audio_receiver) = bounded::<OpenAudioFileCommand>(8);
    let binding = MainPageBinding::new(MainPageDependencies {
        action_handlers: MainPageActionHandlers {
            pick_audio_path: Some(Rc::new(|| Some("C:/music.mp3".to_string()))),
            ..MainPageActionHandlers::default()
        },
        behavior_dependencies: ChoreoMainBehaviorDependencies {
            open_audio_sender: Some(open_audio_sender),
            ..ChoreoMainBehaviorDependencies::default()
        },
    });

    binding.dispatch(ChoreoMainAction::RequestOpenAudio(OpenAudioRequested {
        file_path: String::new(),
        trace_context: None,
    }));

    let forwarded = open_audio_receiver
        .try_recv()
        .expect("picker fallback should forward selected audio file");
    assert_eq!(forwarded.file_path, "C:/music.mp3");
    assert!(binding.state().borrow().is_audio_player_open);
}

#[test]
fn binding_opens_audio_file_and_toggles_play_pause_from_main_audio_actions() {
    let temp_file = unique_temp_file("wav");
    write_test_wav(&temp_file);
    let file_path = temp_file.to_string_lossy().into_owned();

    let binding = MainPageBinding::new(MainPageDependencies::default());

    binding.request_open_audio(OpenAudioRequested {
        file_path: file_path.clone(),
        trace_context: None,
    });

    {
        let state = binding.state();
        let state = state.borrow();
        assert_eq!(
            state.audio_player_state.last_opened_audio_file_path.as_deref(),
            Some(file_path.as_str())
        );
        assert!(state.audio_player_state.has_player);
        assert!(state.is_audio_player_open);
        assert!(!state.audio_player_state.is_playing);
    }

    binding.dispatch(ChoreoMainAction::AudioPlayerAction(
        AudioPlayerAction::TogglePlayPause,
    ));
    let became_playing = wait_until(
        Duration::from_millis(400),
        Duration::from_millis(20),
        || {
            let _ = binding.tick_audio_runtime();
            binding.state().borrow().audio_player_state.is_playing
        },
    );

    let state = binding.state();
    let state = state.borrow();
    assert!(state.audio_player_state.has_player);
    assert!(became_playing);
    assert!(state.audio_player_state.is_playing);

    let _ = fs::remove_file(temp_file);
}

#[test]
fn binding_tick_clears_pending_seek_only_after_runtime_acknowledges_position() {
    let temp_file = unique_temp_file("wav");
    write_test_wav(&temp_file);
    let binding = MainPageBinding::new(MainPageDependencies::default());

    binding.request_open_audio(OpenAudioRequested {
        file_path: temp_file.to_string_lossy().into_owned(),
        trace_context: None,
    });
    binding.dispatch(ChoreoMainAction::AudioPlayerAction(
        AudioPlayerAction::PositionDragStarted,
    ));
    binding.dispatch(ChoreoMainAction::AudioPlayerAction(
        AudioPlayerAction::PositionDragCompleted { position: 0.25 },
    ));

    {
        let state = binding.state();
        let state = state.borrow();
        assert_eq!(state.audio_player_state.pending_seek_position, Some(0.25));
    }

    let acknowledged = wait_until(
        Duration::from_millis(400),
        Duration::from_millis(20),
        || {
            let _ = binding.tick_audio_runtime();
            binding
                .state()
                .borrow()
                .audio_player_state
                .pending_seek_position
                .is_none()
        },
    );

    let state = binding.state();
    let state = state.borrow();
    assert!(acknowledged);
    assert!(state.audio_player_state.pending_seek_position.is_none());
    assert_eq!(state.audio_player_state.position, 0.25);

    let _ = fs::remove_file(temp_file);
}

fn unique_temp_file(extension: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after unix epoch")
        .as_nanos();
    let mut path = std::env::temp_dir();
    path.push(format!("rchoreo_main_binding_{nanos}.{extension}"));
    path
}

fn write_test_wav(path: &Path) {
    let sample_rate = 8_000_u32;
    let sample_count = 8_000_usize;
    let data_size = (sample_count * std::mem::size_of::<i16>()) as u32;
    let mut bytes = Vec::with_capacity(44 + data_size as usize);
    bytes.extend_from_slice(b"RIFF");
    bytes.extend_from_slice(&(36 + data_size).to_le_bytes());
    bytes.extend_from_slice(b"WAVE");
    bytes.extend_from_slice(b"fmt ");
    bytes.extend_from_slice(&16_u32.to_le_bytes());
    bytes.extend_from_slice(&1_u16.to_le_bytes());
    bytes.extend_from_slice(&1_u16.to_le_bytes());
    bytes.extend_from_slice(&sample_rate.to_le_bytes());
    bytes.extend_from_slice(&(sample_rate * 2).to_le_bytes());
    bytes.extend_from_slice(&2_u16.to_le_bytes());
    bytes.extend_from_slice(&16_u16.to_le_bytes());
    bytes.extend_from_slice(b"data");
    bytes.extend_from_slice(&data_size.to_le_bytes());
    for index in 0..sample_count {
        let sample = if index % 32 < 16 {
            i16::MAX / 6
        } else {
            -(i16::MAX / 6)
        };
        bytes.extend_from_slice(&sample.to_le_bytes());
    }
    fs::write(path, bytes).expect("test wav file should be written");
}

fn wait_until(timeout: Duration, interval: Duration, mut predicate: impl FnMut() -> bool) -> bool {
    let start = std::time::Instant::now();
    while start.elapsed() < timeout {
        if predicate() {
            return true;
        }
        thread::sleep(interval);
    }
    predicate()
}
