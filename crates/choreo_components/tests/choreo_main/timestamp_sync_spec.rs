use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use crate::choreo_main;

use choreo_components::audio_player::{
    AudioPlayerBehaviorDependencies, AudioPlayerPositionChangedEvent, AudioPlayerViewModel,
    CloseAudioFileCommand, LinkSceneToPositionCommand, build_audio_player_behaviors,
};
use choreo_components::choreo_main::{MainPageActionHandlers, MainPageBinding, MainPageDependencies};
use choreo_components::global::GlobalProvider;
use choreo_components::preferences::{InMemoryPreferences, Preferences};
use choreo_components::shell;
use choreo_master_mobile_json::SceneId;
use choreo_models::{
    ChoreographyModel, Colors, DancerModel, FloorModel, PositionModel, RoleModel, SceneModel,
};
use crossbeam_channel::{bounded, unbounded};
use slint::Model;

use choreo_main::Report;

fn pump_ui(duration_ms: u64) {
    i_slint_backend_testing::mock_elapsed_time(Duration::from_millis(duration_ms));
    slint::platform::update_timers_and_animations();
}

fn wait_until(timeout: Duration, mut predicate: impl FnMut() -> bool) -> bool {
    let deadline = std::time::Instant::now() + timeout;
    while std::time::Instant::now() < deadline {
        if predicate() {
            return true;
        }
        pump_ui(20);
    }
    predicate()
}

fn build_scene(scene_id: i32, name: &str, timestamp: &str) -> SceneModel {
    SceneModel {
        scene_id: SceneId(scene_id),
        positions: Vec::new(),
        name: name.to_string(),
        text: None,
        fixed_positions: false,
        timestamp: Some(timestamp.to_string()),
        variation_depth: 0,
        variations: Vec::new(),
        current_variation: Vec::new(),
        color: Colors::transparent(),
    }
}

fn build_positioned_scene(
    scene_id: i32,
    name: &str,
    timestamp: &str,
    dancer: Rc<DancerModel>,
    x: f64,
    y: f64,
) -> SceneModel {
    SceneModel {
        scene_id: SceneId(scene_id),
        positions: vec![PositionModel {
            dancer: Some(dancer),
            orientation: None,
            x,
            y,
            curve1_x: None,
            curve1_y: None,
            curve2_x: None,
            curve2_y: None,
            movement1_x: None,
            movement1_y: None,
            movement2_x: None,
            movement2_y: None,
        }],
        name: name.to_string(),
        text: None,
        fixed_positions: true,
        timestamp: Some(timestamp.to_string()),
        variation_depth: 0,
        variations: Vec::new(),
        current_variation: Vec::new(),
        color: Colors::transparent(),
    }
}

fn run_in_ui_thread(test: impl FnOnce() + Send + 'static) {
    let handle = std::thread::Builder::new()
        .name("timestamp-sync-spec".to_string())
        .stack_size(8 * 1024 * 1024)
        .spawn(test)
        .expect("spec thread should start");
    match handle.join() {
        Ok(()) => {}
        Err(error) => {
            if let Some(message) = error.downcast_ref::<String>() {
                panic!("{message}");
            }
            if let Some(message) = error.downcast_ref::<&str>() {
                panic!("{message}");
            }
            panic!("spec thread panicked");
        }
    }
}

#[test]
#[serial_test::serial]
fn timestamp_sync_spec() {
    let suite = rspec::describe("timestamp synchronization", (), |spec| {
        spec.it(
            "updates slider and floor scene when selecting a timestamped scene from list",
            |_| {
                run_in_ui_thread(|| {
                    i_slint_backend_testing::init_no_event_loop();

                    let ui = shell::create_shell_host().expect("shell should be created");
                    let global_provider = GlobalProvider::new();
                    let global_state = global_provider.global_state();
                    let global_state_store = global_provider.global_state_store();
                    let state_machine = global_provider.state_machine();
                    let preferences: Rc<dyn Preferences> = Rc::new(InMemoryPreferences::new());

                    let choreography = ChoreographyModel {
                        scenes: vec![
                            build_scene(1, "Scene 1", "5"),
                            build_scene(2, "Scene 2", "10"),
                        ],
                        ..ChoreographyModel::default()
                    };
                    let _ = global_state_store.try_update(|state| {
                        state.choreography = choreography;
                    });

                    let (open_audio_sender, open_audio_receiver) = bounded(1);
                    let (close_audio_sender, close_audio_receiver) = bounded::<CloseAudioFileCommand>(1);
                    let (audio_position_sender_for_scenes, audio_position_receiver_for_scenes) =
                        bounded::<AudioPlayerPositionChangedEvent>(16);
                    let (audio_position_sender_for_floor, audio_position_receiver_for_floor) =
                        bounded::<AudioPlayerPositionChangedEvent>(16);
                    let (link_scene_sender, link_scene_receiver) =
                        bounded::<LinkSceneToPositionCommand>(1);
                    let behaviors = build_audio_player_behaviors(AudioPlayerBehaviorDependencies {
                        global_state_store: Rc::clone(&global_state_store),
                        open_audio_receiver,
                        close_audio_receiver,
                        position_changed_senders: vec![
                            audio_position_sender_for_scenes,
                            audio_position_sender_for_floor,
                        ],
                        link_scene_receiver,
                        preferences: Rc::clone(&preferences),
                    });
                    let audio_player = Rc::new(RefCell::new(AudioPlayerViewModel::new(
                        None,
                        link_scene_sender,
                        behaviors,
                    )));
                    let (scenes_show_dialog_sender, _scenes_show_dialog_receiver) = unbounded();
                    let (scenes_close_dialog_sender, _scenes_close_dialog_receiver) = unbounded();
                    let (redraw_floor_sender, redraw_floor_receiver) = unbounded();

                    let binding = MainPageBinding::new(
                        ui,
                        MainPageDependencies {
                            global_state,
                            global_state_store,
                            state_machine,
                            audio_player,
                            haptic_feedback: None,
                            open_audio_sender,
                            close_audio_sender,
                            audio_position_receiver_for_scenes,
                            audio_position_receiver_for_floor,
                            scenes_show_dialog_sender,
                            scenes_close_dialog_sender,
                            redraw_floor_sender,
                            redraw_floor_receiver,
                            preferences,
                            actions: MainPageActionHandlers::default(),
                        },
                    );

                    let view = binding.view();
                    let selected = wait_until(Duration::from_secs(1), || {
                        view.get_scenes().row_count() >= 2
                    });
                    assert!(selected);

                    view.invoke_scenes_select_scene(1);
                    let synced = wait_until(Duration::from_secs(1), || {
                        (view.get_audio_position() - 10.0).abs() < 0.01
                            && view.get_floor_scene_name() == "Scene 2"
                    });
                    assert!(synced);
                });
            },
        );

        spec.it(
            "updates selected scene and floor when slider timestamp moves into next range",
            |_| {
                run_in_ui_thread(|| {
                    i_slint_backend_testing::init_no_event_loop();

                    let ui = shell::create_shell_host().expect("shell should be created");
                    let global_provider = GlobalProvider::new();
                    let global_state = global_provider.global_state();
                    let global_state_store = global_provider.global_state_store();
                    let state_machine = global_provider.state_machine();
                    let preferences: Rc<dyn Preferences> = Rc::new(InMemoryPreferences::new());

                    let choreography = ChoreographyModel {
                        scenes: vec![
                            build_scene(1, "Scene 1", "5"),
                            build_scene(2, "Scene 2", "10"),
                            build_scene(3, "Scene 3", "20"),
                        ],
                        ..ChoreographyModel::default()
                    };
                    let _ = global_state_store.try_update(|state| {
                        state.choreography = choreography;
                    });

                    let (open_audio_sender, open_audio_receiver) = bounded(1);
                    let (close_audio_sender, close_audio_receiver) = bounded::<CloseAudioFileCommand>(1);
                    let (audio_position_sender_for_scenes, audio_position_receiver_for_scenes) =
                        bounded::<AudioPlayerPositionChangedEvent>(16);
                    let (audio_position_sender_for_floor, audio_position_receiver_for_floor) =
                        bounded::<AudioPlayerPositionChangedEvent>(16);
                    let (link_scene_sender, link_scene_receiver) =
                        bounded::<LinkSceneToPositionCommand>(1);
                    let behaviors = build_audio_player_behaviors(AudioPlayerBehaviorDependencies {
                        global_state_store: Rc::clone(&global_state_store),
                        open_audio_receiver,
                        close_audio_receiver,
                        position_changed_senders: vec![
                            audio_position_sender_for_scenes,
                            audio_position_sender_for_floor,
                        ],
                        link_scene_receiver,
                        preferences: Rc::clone(&preferences),
                    });
                    let audio_player = Rc::new(RefCell::new(AudioPlayerViewModel::new(
                        None,
                        link_scene_sender,
                        behaviors,
                    )));
                    let (scenes_show_dialog_sender, _scenes_show_dialog_receiver) = unbounded();
                    let (scenes_close_dialog_sender, _scenes_close_dialog_receiver) = unbounded();
                    let (redraw_floor_sender, redraw_floor_receiver) = unbounded();

                    let binding = MainPageBinding::new(
                        ui,
                        MainPageDependencies {
                            global_state,
                            global_state_store,
                            state_machine,
                            audio_player,
                            haptic_feedback: None,
                            open_audio_sender,
                            close_audio_sender,
                            audio_position_receiver_for_scenes,
                            audio_position_receiver_for_floor,
                            scenes_show_dialog_sender,
                            scenes_close_dialog_sender,
                            redraw_floor_sender,
                            redraw_floor_receiver,
                            preferences,
                            actions: MainPageActionHandlers::default(),
                        },
                    );

                    let view = binding.view();
                    let ready = wait_until(Duration::from_secs(1), || {
                        view.get_scenes().row_count() >= 3
                    });
                    assert!(ready);

                    view.invoke_audio_position_changed(12.0);
                    let synced = wait_until(Duration::from_secs(1), || {
                        view.get_floor_scene_name() == "Scene 2"
                    });
                    assert!(synced);
                });
            },
        );

        spec.it(
            "keeps slider position unchanged when selecting a scene without timestamp",
            |_| {
                run_in_ui_thread(|| {
                    i_slint_backend_testing::init_no_event_loop();

                    let ui = shell::create_shell_host().expect("shell should be created");
                    let global_provider = GlobalProvider::new();
                    let global_state = global_provider.global_state();
                    let global_state_store = global_provider.global_state_store();
                    let state_machine = global_provider.state_machine();
                    let preferences: Rc<dyn Preferences> = Rc::new(InMemoryPreferences::new());

                    let choreography = ChoreographyModel {
                        scenes: vec![
                            build_scene(1, "Scene 1", "5"),
                            SceneModel {
                                scene_id: SceneId(2),
                                positions: Vec::new(),
                                name: "Scene 2".to_string(),
                                text: None,
                                fixed_positions: false,
                                timestamp: None,
                                variation_depth: 0,
                                variations: Vec::new(),
                                current_variation: Vec::new(),
                                color: Colors::transparent(),
                            },
                        ],
                        ..ChoreographyModel::default()
                    };
                    let _ = global_state_store.try_update(|state| {
                        state.choreography = choreography;
                    });

                    let (open_audio_sender, open_audio_receiver) = bounded(1);
                    let (close_audio_sender, close_audio_receiver) = bounded::<CloseAudioFileCommand>(1);
                    let (audio_position_sender_for_scenes, audio_position_receiver_for_scenes) =
                        bounded::<AudioPlayerPositionChangedEvent>(16);
                    let (audio_position_sender_for_floor, audio_position_receiver_for_floor) =
                        bounded::<AudioPlayerPositionChangedEvent>(16);
                    let (link_scene_sender, link_scene_receiver) =
                        bounded::<LinkSceneToPositionCommand>(1);
                    let behaviors = build_audio_player_behaviors(AudioPlayerBehaviorDependencies {
                        global_state_store: Rc::clone(&global_state_store),
                        open_audio_receiver,
                        close_audio_receiver,
                        position_changed_senders: vec![
                            audio_position_sender_for_scenes,
                            audio_position_sender_for_floor,
                        ],
                        link_scene_receiver,
                        preferences: Rc::clone(&preferences),
                    });
                    let audio_player = Rc::new(RefCell::new(AudioPlayerViewModel::new(
                        None,
                        link_scene_sender,
                        behaviors,
                    )));
                    let (scenes_show_dialog_sender, _scenes_show_dialog_receiver) = unbounded();
                    let (scenes_close_dialog_sender, _scenes_close_dialog_receiver) = unbounded();
                    let (redraw_floor_sender, redraw_floor_receiver) = unbounded();

                    let binding = MainPageBinding::new(
                        ui,
                        MainPageDependencies {
                            global_state,
                            global_state_store,
                            state_machine,
                            audio_player,
                            haptic_feedback: None,
                            open_audio_sender,
                            close_audio_sender,
                            audio_position_receiver_for_scenes,
                            audio_position_receiver_for_floor,
                            scenes_show_dialog_sender,
                            scenes_close_dialog_sender,
                            redraw_floor_sender,
                            redraw_floor_receiver,
                            preferences,
                            actions: MainPageActionHandlers::default(),
                        },
                    );

                    let view = binding.view();
                    assert!(wait_until(Duration::from_secs(1), || {
                        view.get_scenes().row_count() >= 2
                    }));

                    assert!(wait_until(Duration::from_secs(1), || {
                        (view.get_audio_position() - 5.0).abs() < 0.01
                    }));

                    view.invoke_scenes_select_scene(1);
                    let synced = wait_until(Duration::from_secs(1), || {
                        (view.get_audio_position() - 5.0).abs() < 0.01
                            && view.get_floor_scene_name() == "Scene 2"
                    });
                    assert!(
                        synced,
                        "audio_position={}, floor_scene_name={}",
                        view.get_audio_position(),
                        view.get_floor_scene_name()
                    );
                });
            },
        );

        spec.it(
            "interpolates floor positions when slider timestamp is between timestamped scenes",
            |_| {
                run_in_ui_thread(|| {
                    i_slint_backend_testing::init_no_event_loop();

                    let ui = shell::create_shell_host().expect("shell should be created");
                    let global_provider = GlobalProvider::new();
                    let global_state = global_provider.global_state();
                    let global_state_store = global_provider.global_state_store();
                    let state_machine = global_provider.state_machine();
                    let preferences: Rc<dyn Preferences> = Rc::new(InMemoryPreferences::new());

                    let role = Rc::new(RoleModel {
                        z_index: 0,
                        name: "Lead".to_string(),
                        color: Colors::red(),
                    });
                    let dancer = Rc::new(DancerModel {
                        dancer_id: choreo_master_mobile_json::DancerId(1),
                        role,
                        name: "Alex".to_string(),
                        shortcut: "A".to_string(),
                        color: Colors::red(),
                        icon: None,
                    });

                    let choreography = ChoreographyModel {
                        floor: FloorModel {
                            size_front: 5,
                            size_back: 5,
                            size_left: 5,
                            size_right: 5,
                        },
                        dancers: vec![Rc::clone(&dancer)],
                        scenes: vec![
                            build_positioned_scene(1, "Scene 1", "0", Rc::clone(&dancer), 0.0, 0.0),
                            build_positioned_scene(2, "Scene 2", "10", dancer, 10.0, 0.0),
                        ],
                        ..ChoreographyModel::default()
                    };
                    let _ = global_state_store.try_update(|state| {
                        state.choreography = choreography;
                    });

                    let (open_audio_sender, open_audio_receiver) = bounded(1);
                    let (close_audio_sender, close_audio_receiver) = bounded::<CloseAudioFileCommand>(1);
                    let (audio_position_sender_for_scenes, audio_position_receiver_for_scenes) =
                        bounded::<AudioPlayerPositionChangedEvent>(16);
                    let (audio_position_sender_for_floor, audio_position_receiver_for_floor) =
                        bounded::<AudioPlayerPositionChangedEvent>(16);
                    let (link_scene_sender, link_scene_receiver) =
                        bounded::<LinkSceneToPositionCommand>(1);
                    let behaviors = build_audio_player_behaviors(AudioPlayerBehaviorDependencies {
                        global_state_store: Rc::clone(&global_state_store),
                        open_audio_receiver,
                        close_audio_receiver,
                        position_changed_senders: vec![
                            audio_position_sender_for_scenes,
                            audio_position_sender_for_floor,
                        ],
                        link_scene_receiver,
                        preferences: Rc::clone(&preferences),
                    });
                    let audio_player = Rc::new(RefCell::new(AudioPlayerViewModel::new(
                        None,
                        link_scene_sender,
                        behaviors,
                    )));
                    let (scenes_show_dialog_sender, _scenes_show_dialog_receiver) = unbounded();
                    let (scenes_close_dialog_sender, _scenes_close_dialog_receiver) = unbounded();
                    let (redraw_floor_sender, redraw_floor_receiver) = unbounded();

                    let binding = MainPageBinding::new(
                        ui,
                        MainPageDependencies {
                            global_state,
                            global_state_store,
                            state_machine,
                            audio_player,
                            haptic_feedback: None,
                            open_audio_sender,
                            close_audio_sender,
                            audio_position_receiver_for_scenes,
                            audio_position_receiver_for_floor,
                            scenes_show_dialog_sender,
                            scenes_close_dialog_sender,
                            redraw_floor_sender,
                            redraw_floor_receiver,
                            preferences,
                            actions: MainPageActionHandlers::default(),
                        },
                    );

                    let view = binding.view();
                    assert!(wait_until(Duration::from_secs(1), || {
                        view.get_scenes().row_count() >= 2
                    }));

                    view.invoke_audio_position_changed(5.0);
                    let synced = wait_until(Duration::from_secs(1), || {
                        let positions = view.get_floor_positions();
                        if positions.row_count() == 0 {
                            return false;
                        }
                        positions
                            .row_data(0)
                            .map(|p| (p.x as f64 - 5.0).abs() < 0.0001)
                            .unwrap_or(false)
                    });
                    assert!(synced);
                });
            },
        );
    });

    let report = choreo_main::run_suite(&suite);
    assert!(report.is_success());
}
