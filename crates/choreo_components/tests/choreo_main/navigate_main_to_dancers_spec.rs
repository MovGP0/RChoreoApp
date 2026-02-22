use std::cell::RefCell;
use std::rc::Rc;
use std::time::{Duration, Instant};

use crate::choreo_main;

use choreo_components::audio_player::{
    AudioPlayerBehaviorDependencies, AudioPlayerPositionChangedEvent, AudioPlayerViewModel,
    CloseAudioFileCommand, LinkSceneToPositionCommand, build_audio_player_behaviors,
};
use choreo_components::choreo_main::{MainPageActionHandlers, MainPageBinding, MainPageDependencies};
use choreo_components::global::GlobalProvider;
use choreo_components::preferences::{InMemoryPreferences, Preferences};
use choreo_components::scenes::SceneViewModel;
use choreo_components::shell;
use choreo_components::ScenesInfo;
use choreo_master_mobile_json::{DancerId, SceneId};
use choreo_models::{Colors, DancerModel, PositionModel, RoleModel, SceneModel};
use crossbeam_channel::{bounded, unbounded};
use choreo_main::Report;
use slint::ComponentHandle;
use slint::Model;

fn create_binding() -> (MainPageBinding, Rc<choreo_components::global::GlobalStateActor>) {
    let ui = shell::create_shell_host().expect("shell should be created");
    let global_provider = GlobalProvider::new();
    let global_state = global_provider.global_state();
    let global_state_store = global_provider.global_state_store();
    let state_machine = global_provider.state_machine();
    let seeded = global_state_store.try_update(|state| {
        let gentleman_role = Rc::new(RoleModel {
            z_index: 0,
            name: "Gentleman".to_string(),
            color: Colors::transparent(),
        });
        let lady_role = Rc::new(RoleModel {
            z_index: 1,
            name: "Lady".to_string(),
            color: Colors::transparent(),
        });
        let dancer = Rc::new(DancerModel {
            dancer_id: DancerId(1),
            role: gentleman_role.clone(),
            name: "Alice".to_string(),
            shortcut: "A".to_string(),
            color: choreo_master_mobile_json::Color {
                r: 1,
                g: 2,
                b: 3,
                a: 255,
            },
            icon: None,
        });
        let second_dancer = Rc::new(DancerModel {
            dancer_id: DancerId(2),
            role: lady_role.clone(),
            name: "Betty".to_string(),
            shortcut: "B".to_string(),
            color: choreo_master_mobile_json::Color {
                r: 10,
                g: 20,
                b: 30,
                a: 255,
            },
            icon: None,
        });
        let scene_model = SceneModel {
            scene_id: SceneId(1),
            positions: vec![PositionModel {
                dancer: Some(dancer.clone()),
                orientation: None,
                x: 0.0,
                y: 0.0,
                curve1_x: None,
                curve1_y: None,
                curve2_x: None,
                curve2_y: None,
                movement1_x: None,
                movement1_y: None,
                movement2_x: None,
                movement2_y: None,
            }],
            name: "Scene 1".to_string(),
            text: None,
            fixed_positions: false,
            timestamp: None,
            variation_depth: 0,
            variations: Vec::new(),
            current_variation: Vec::new(),
            color: Colors::transparent(),
        };
        let scene_view_model = SceneViewModel {
            scene_id: scene_model.scene_id,
            name: scene_model.name.clone(),
            text: "".to_string(),
            fixed_positions: scene_model.fixed_positions,
            timestamp: None,
            is_selected: true,
            positions: scene_model.positions.clone(),
            variation_depth: scene_model.variation_depth,
            variations: scene_model.variations.clone(),
            current_variation: scene_model.current_variation.clone(),
            color: scene_model.color.clone(),
        };
        state.choreography.roles = vec![gentleman_role, lady_role];
        state.choreography.dancers = vec![dancer, second_dancer];
        state.choreography.scenes = vec![scene_model];
        state.scenes = vec![scene_view_model.clone()];
        state.selected_scene = Some(scene_view_model);
    });
    assert!(seeded, "failed to seed global state");
    let preferences: Rc<dyn Preferences> = Rc::new(InMemoryPreferences::new());

    let (open_audio_sender, open_audio_receiver) = bounded(1);
    let (close_audio_sender, close_audio_receiver) = bounded::<CloseAudioFileCommand>(1);
    let (audio_position_sender_for_scenes, audio_position_receiver_for_scenes) =
        bounded::<AudioPlayerPositionChangedEvent>(16);
    let (audio_position_sender_for_floor, audio_position_receiver_for_floor) =
        bounded::<AudioPlayerPositionChangedEvent>(16);
    let (link_scene_sender, link_scene_receiver) = bounded::<LinkSceneToPositionCommand>(1);
    let behaviors = build_audio_player_behaviors(AudioPlayerBehaviorDependencies {
        global_state_store: Rc::clone(&global_state_store),
        open_audio_receiver,
        close_audio_receiver,
        position_changed_senders: vec![audio_position_sender_for_scenes, audio_position_sender_for_floor],
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
            global_state_store: Rc::clone(&global_state_store),
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

    (binding, global_state_store)
}

fn run_in_ui_thread(test: impl FnOnce() + Send + 'static) {
    let handle = std::thread::Builder::new()
        .name("navigate-main-to-dancers-spec".to_string())
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

fn wait_until(timeout: Duration, mut predicate: impl FnMut() -> bool) -> bool {
    let deadline = Instant::now() + timeout;
    while Instant::now() < deadline {
        if predicate() {
            return true;
        }
        i_slint_backend_testing::mock_elapsed_time(Duration::from_millis(20));
        slint::platform::update_timers_and_animations();
    }

    predicate()
}

#[test]
#[serial_test::serial]
fn navigate_main_to_dancers_spec() {
    let suite = rspec::describe("navigate from main page to dancers page", (), |spec| {
        spec.it("shows the dancers page when scenes requests dancer settings navigation", |_| {
            run_in_ui_thread(|| {
                i_slint_backend_testing::init_no_event_loop();

                let (binding, _global_state_store) = create_binding();
                let view = binding.view();
                let scenes_info = view.global::<ScenesInfo<'_>>();

                assert_eq!(view.get_content_index(), 0);
                scenes_info.invoke_navigate_to_dancer_settings();
                assert_eq!(view.get_content_index(), 2);
            });
        });

        spec.it(
            "loads dancer list items into the dancers settings page when opened",
            |_| {
                run_in_ui_thread(|| {
                    i_slint_backend_testing::init_no_event_loop();

                    let (binding, _global_state_store) = create_binding();
                    let view = binding.view();
                    let scenes_info = view.global::<ScenesInfo<'_>>();

                    scenes_info.invoke_navigate_to_dancer_settings();
                    let dancer_items = view.get_dancer_settings_dancer_items();
                    assert!(dancer_items.row_count() >= 1);
                });
            },
        );

        spec.it(
            "updates role and color editor bindings when selecting a different dancer",
            |_| {
                run_in_ui_thread(|| {
                    i_slint_backend_testing::init_no_event_loop();

                    let (binding, _global_state_store) = create_binding();
                    let view = binding.view();
                    let scenes_info = view.global::<ScenesInfo<'_>>();

                    scenes_info.invoke_navigate_to_dancer_settings();

                    let loaded = wait_until(Duration::from_secs(1), || {
                        view.get_dancer_settings_dancer_items().row_count() >= 1
                            && view.get_dancer_settings_role_options().row_count() >= 2
                    });
                    assert!(loaded, "dancer settings should be loaded");

                    view.invoke_dancer_settings_select_dancer(0);
                    let selected_first = wait_until(Duration::from_secs(1), || {
                        view.get_dancer_settings_selected_role_index() == 0
                    });
                    assert!(selected_first);
                    assert_eq!(view.get_dancer_settings_dancer_color().red(), 1);
                    assert_eq!(view.get_dancer_settings_dancer_color().green(), 2);
                    assert_eq!(view.get_dancer_settings_dancer_color().blue(), 3);

                    view.invoke_dancer_settings_select_dancer(1);
                    let selected_second = wait_until(Duration::from_secs(1), || {
                        view.get_dancer_settings_selected_role_index() == 1
                            && view.get_dancer_settings_dancer_color().red() == 10
                            && view.get_dancer_settings_dancer_color().green() == 20
                            && view.get_dancer_settings_dancer_color().blue() == 30
                    });
                    assert!(selected_second, "role and color should reflect selected dancer");
                });
            },
        );

    });

    let report = choreo_main::run_suite(&suite);
    assert!(report.is_success());
}
