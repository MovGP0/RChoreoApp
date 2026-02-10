use std::cell::RefCell;
use std::rc::Rc;

use crate::choreo_main;

use choreo_components::audio_player::{
    AudioPlayerBehaviorDependencies, AudioPlayerPositionChangedEvent, AudioPlayerViewModel,
    CloseAudioFileCommand, LinkSceneToPositionCommand, build_audio_player_behaviors,
};
use choreo_components::choreo_main::{MainPageActionHandlers, MainPageBinding, MainPageDependencies};
use choreo_components::global::GlobalProvider;
use choreo_components::preferences::{InMemoryPreferences, Preferences};
use choreo_components::shell;
use choreo_master_mobile_json::DancerId;
use choreo_models::{Colors, DancerModel, RoleModel};
use crossbeam_channel::{bounded, unbounded};
use choreo_main::Report;
use slint::Model;

fn create_binding() -> MainPageBinding {
    let ui = shell::create_shell_host().expect("shell should be created");
    let global_provider = GlobalProvider::new();
    let global_state = global_provider.global_state();
    let global_state_store = global_provider.global_state_store();
    let state_machine = global_provider.state_machine();
    let seeded = global_state_store.try_update(|state| {
        let role = Rc::new(RoleModel {
            z_index: 0,
            name: "Lead".to_string(),
            color: Colors::transparent(),
        });
        let dancer = Rc::new(DancerModel {
            dancer_id: DancerId(1),
            role: role.clone(),
            name: "Alice".to_string(),
            shortcut: "A".to_string(),
            color: Colors::transparent(),
            icon: None,
        });
        state.choreography.roles = vec![role];
        state.choreography.dancers = vec![dancer];
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

    MainPageBinding::new(
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
    )
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

#[test]
#[serial_test::serial]
fn navigate_main_to_dancers_spec() {
    let suite = rspec::describe("navigate from main page to dancers page", (), |spec| {
        spec.it("shows the dancers page when scenes requests dancer settings navigation", |_| {
            run_in_ui_thread(|| {
                i_slint_backend_testing::init_no_event_loop();

                let binding = create_binding();
                let view = binding.view();

                assert_eq!(view.get_content_index(), 0);
                view.invoke_scenes_navigate_to_dancer_settings();
                assert_eq!(view.get_content_index(), 2);
            });
        });

        spec.it(
            "loads dancer list items into the dancers settings page when opened",
            |_| {
                run_in_ui_thread(|| {
                    i_slint_backend_testing::init_no_event_loop();

                    let binding = create_binding();
                    let view = binding.view();

                    view.invoke_scenes_navigate_to_dancer_settings();
                    let dancer_items = view.get_dancer_settings_dancer_items();
                    assert!(dancer_items.row_count() >= 1);
                });
            },
        );
    });

    let report = choreo_main::run_suite(&suite);
    assert!(report.is_success());
}
