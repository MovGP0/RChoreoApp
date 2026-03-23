use std::cell::RefCell;
use std::rc::Rc;
use std::sync::mpsc::sync_channel;

use crate::choreo_main::Report;
use choreo_components::audio_player::OpenAudioFileCommand;
use choreo_components::choreo_main::MainBehaviorDependencies;
use choreo_components::choreo_main::MainPageBinding;
use choreo_components::choreo_main::MainPageDependencies;
use choreo_components::choreo_main::OpenAudioRequested;
use choreo_components::choreo_main::OpenImageRequested;
use choreo_components::choreo_main::actions::ChoreoMainAction;
use choreo_components::global::GlobalStateActor;
use choreo_components::global::GlobalStateModel;
use choreo_components::preferences::InMemoryPreferences;
use choreo_components::preferences::Preferences;
use choreo_models::SettingsPreferenceKeys;
use choreo_state_machine::ApplicationStateMachine;
use choreo_state_machine::StateKind;
use crossbeam_channel::bounded;

#[test]
fn non_ui_behavior_parity_spec() {
    let suite =
        rspec::describe("choreo_main non-ui behavior parity", (), |spec| {
            spec.it(
                "forwards open-audio requests to audio command sender",
                |_| {
                    let (open_audio_sender, open_audio_receiver) =
                        bounded::<OpenAudioFileCommand>(8);
                    let binding = MainPageBinding::new(MainPageDependencies {
                        behavior_dependencies: MainBehaviorDependencies {
                            open_audio_sender: Some(open_audio_sender),
                            ..MainBehaviorDependencies::default()
                        },
                        ..MainPageDependencies::default()
                    });

                    binding.request_open_audio(OpenAudioRequested {
                        file_path: "C:/song.mp3".to_string(),
                        trace_context: None,
                    });

                    let forwarded = open_audio_receiver
                        .try_recv()
                        .expect("open-audio command should be forwarded");
                    assert_eq!(forwarded.file_path, "C:/song.mp3");
                },
            );

            spec.it(
            "applies open-image through open-svg behavior with global/preference/draw side effects",
            |_| {
                let global_state_store = GlobalStateActor::new();
                let preferences: Rc<dyn Preferences> = Rc::new(InMemoryPreferences::new());
                let (draw_floor_sender, draw_floor_receiver) = sync_channel(8);
                let binding = MainPageBinding::new(MainPageDependencies {
                    behavior_dependencies: MainBehaviorDependencies {
                        global_state_store: Some(Rc::clone(&global_state_store)),
                        preferences: Some(Rc::clone(&preferences)),
                        draw_floor_sender: Some(draw_floor_sender),
                        ..MainBehaviorDependencies::default()
                    },
                    ..MainPageDependencies::default()
                });

                binding.request_open_image(OpenImageRequested {
                    file_path: "C:/floor.svg".to_string(),
                });

                let state = binding.view_model();
                assert_eq!(state.borrow().state().svg_file_path.as_deref(), Some("C:/floor.svg"));
                global_state_store.drain();
                let global_svg = global_state_store
                    .try_with_state(|state| state.svg_file_path.clone())
                    .expect("global state should be readable");
                assert_eq!(global_svg.as_deref(), Some("C:/floor.svg"));
                assert_eq!(
                    preferences.get_string(SettingsPreferenceKeys::LAST_OPENED_SVG_FILE, ""),
                    "C:/floor.svg"
                );
                assert!(draw_floor_receiver.try_recv().is_ok());
            },
        );

            spec.it(
                "applies interaction mode transitions to the state machine",
                |_| {
                    let global_state_store = GlobalStateActor::new();
                    let state_machine = Rc::new(RefCell::new(
                        ApplicationStateMachine::with_default_transitions(Box::new(
                            GlobalStateModel::default(),
                        )),
                    ));
                    let binding = MainPageBinding::new(MainPageDependencies {
                        behavior_dependencies: MainBehaviorDependencies {
                            global_state_store: Some(global_state_store),
                            state_machine: Some(Rc::clone(&state_machine)),
                            ..MainBehaviorDependencies::default()
                        },
                        ..MainPageDependencies::default()
                    });

                    binding.dispatch(ChoreoMainAction::SelectMode { index: 1 });

                    assert_eq!(
                        state_machine.borrow().state().kind(),
                        StateKind::MovePositionsState
                    );
                },
            );
        });

    let report = crate::choreo_main::run_suite(&suite);
    assert!(report.is_success());
}
