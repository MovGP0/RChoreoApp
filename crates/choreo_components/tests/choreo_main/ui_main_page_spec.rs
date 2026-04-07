use crate::choreo_main::Report;
use crate::choreo_main::actions::ChoreoMainAction;
use crate::choreo_main::reducer::reduce;
use crate::choreo_main::state::ChoreoMainState;
use crate::choreo_main::state::InteractionMode;
use crate::choreo_main::state::MainContent;
use crate::choreo_main::ui::home_icon_name;
use crate::choreo_main::ui::mode_count;
use crate::choreo_main::ui::mode_label;
use crate::choreo_main::ui::nav_icon_name;
use crate::choreo_main::ui::open_audio_icon_name;
use crate::choreo_main::ui::open_image_icon_name;
use crate::choreo_main::ui::top_bar_nav_action;
use crate::choreo_main::ui::top_bar_open_audio_action;
use crate::choreo_main::ui::top_bar_settings_action;
use crate::choreo_main::ui::top_bar_settings_icon_name;
use choreo_components::choreo_main::actions::OpenAudioRequested;
use choreo_components::choreography_settings::actions::ChoreographySettingsAction;
use choreo_components::choreography_settings::actions::UpdateSelectedSceneAction;
use choreo_components::dancers::actions::DancersAction;
use choreo_components::dancers::state::DancerState;
use choreo_components::dancers::state::RoleState;
use choreo_components::dancers::state::transparent_color;
use choreo_components::settings::actions::SettingsAction;
use choreo_components::settings::state::AudioPlayerBackend;

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
fn ui_main_page_spec() {
    let suite = rspec::describe("main page egui view model", (), |spec| {
        spec.it("draws without panic for the default state", |_| {
            let state = ChoreoMainState::default();
            let context = egui::Context::default();

            let _ = context.run(egui::RawInput::default(), |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    let _ = crate::choreo_main::ui::draw(ui, &state);
                });
            });
        });

        spec.it("exposes all main page interaction labels", |_| {
            let mut errors = Vec::new();

            check_eq!(errors, mode_count(), 6);
            check_eq!(errors, mode_label(0), "View");
            check_eq!(errors, mode_label(1), "Move");
            check_eq!(errors, mode_label(2), "Rotate around center");
            check_eq!(errors, mode_label(3), "Rotate around dancer");
            check_eq!(errors, mode_label(4), "Scale");
            check_eq!(errors, mode_label(5), "Line of sight");

            assert_no_errors(errors);
        });

        spec.it(
            "maps top bar toggles and audio open to parity actions",
            |_| {
                let mut errors = Vec::new();

                check_eq!(
                    errors,
                    top_bar_nav_action(false),
                    ChoreoMainAction::ToggleNav
                );
                check_eq!(errors, top_bar_nav_action(true), ChoreoMainAction::CloseNav);
                check_eq!(
                    errors,
                    top_bar_settings_action(false),
                    ChoreoMainAction::OpenSettings
                );
                check_eq!(
                    errors,
                    top_bar_settings_action(true),
                    ChoreoMainAction::CloseSettings
                );
                check_eq!(
                    errors,
                    top_bar_open_audio_action(),
                    ChoreoMainAction::RequestOpenAudio(OpenAudioRequested {
                        file_path: String::new(),
                        trace_context: None,
                    })
                );

                assert_no_errors(errors);
            },
        );

        spec.it("uses parity icon tokens for top bar actions", |_| {
            let mut errors = Vec::new();

            check_eq!(errors, nav_icon_name(false), "menu");
            check_eq!(errors, nav_icon_name(true), "close");
            check_eq!(errors, top_bar_settings_icon_name(), "edit");
            check_eq!(errors, home_icon_name(), "home");
            check_eq!(errors, open_image_icon_name(), "image");
            check_eq!(errors, open_audio_icon_name(), "play_circle");

            assert_no_errors(errors);
        });

        spec.it(
            "updates mode index and interaction mode from a selected menu item",
            |_| {
                let mut state = ChoreoMainState::default();
                reduce(&mut state, ChoreoMainAction::SelectMode { index: 2 });

                let mut errors = Vec::new();

                check_eq!(errors, state.selected_mode_index, 2);
                check_eq!(
                    errors,
                    state.interaction_mode,
                    InteractionMode::RotateAroundCenter
                );

                assert_no_errors(errors);
            },
        );

        spec.it(
            "tracks drawer and audio panel open and close actions",
            |_| {
                let mut state = ChoreoMainState::default();
                reduce(&mut state, ChoreoMainAction::ToggleNav);
                reduce(&mut state, ChoreoMainAction::OpenSettings);
                reduce(&mut state, ChoreoMainAction::OpenAudioPanel);

                let mut errors = Vec::new();

                check!(errors, state.is_nav_open);
                check!(errors, state.is_choreography_settings_open);
                check!(errors, state.is_audio_player_open);

                reduce(&mut state, ChoreoMainAction::CloseNav);
                reduce(&mut state, ChoreoMainAction::CloseSettings);
                reduce(&mut state, ChoreoMainAction::CloseAudioPanel);

                check!(errors, !state.is_nav_open);
                check!(errors, !state.is_choreography_settings_open);
                check!(errors, !state.is_audio_player_open);

                assert_no_errors(errors);
            },
        );

        spec.it(
            "syncs choreography settings projection from main scene selection",
            |_| {
                let mut state = ChoreoMainState::default();
                reduce(
                    &mut state,
                    ChoreoMainAction::SetScenes {
                        scenes: vec![
                            crate::choreo_main::state::SceneState {
                                name: "Intro".to_string(),
                                timestamp_seconds: Some(1.5),
                            },
                            crate::choreo_main::state::SceneState {
                                name: "Finale".to_string(),
                                timestamp_seconds: Some(3.0),
                            },
                        ],
                    },
                );
                reduce(&mut state, ChoreoMainAction::SelectScene { index: 1 });

                let mut errors = Vec::new();

                check_eq!(
                    errors,
                    state.choreography_settings_state.scene_name,
                    "Finale"
                );
                check!(
                    errors,
                    state.choreography_settings_state.scene_has_timestamp
                );
                check!(
                    errors,
                    (state.choreography_settings_state.scene_timestamp_seconds - 3.0).abs()
                        < 0.0001
                );

                assert_no_errors(errors);
            },
        );

        spec.it(
            "routes choreography settings drawer actions back into shared main state",
            |_| {
                let mut state = ChoreoMainState::default();
                reduce(
                    &mut state,
                    ChoreoMainAction::SetScenes {
                        scenes: vec![crate::choreo_main::state::SceneState {
                            name: "Intro".to_string(),
                            timestamp_seconds: Some(1.5),
                        }],
                    },
                );

                reduce(
                    &mut state,
                    ChoreoMainAction::ChoreographySettingsAction(
                        ChoreographySettingsAction::UpdateSelectedScene(
                            UpdateSelectedSceneAction::SceneName("Updated".to_string()),
                        ),
                    ),
                );

                let mut errors = Vec::new();

                check_eq!(
                    errors,
                    state.choreography_settings_state.scene_name,
                    "Updated"
                );
                check_eq!(errors, state.scenes[0].name, "Updated");
                check_eq!(errors, state.floor_scene_name.as_deref(), Some("Updated"));
                check_eq!(errors, state.draw_floor_request_count, 1);

                assert_no_errors(errors);
            },
        );

        spec.it(
            "routes dancers content through dancers pane and dispatches dancer actions",
            |_| {
                let mut state = ChoreoMainState {
                    content: MainContent::Dancers,
                    ..ChoreoMainState::default()
                };
                state.dancers_state.dancers = vec![DancerState {
                    dancer_id: 1,
                    role: RoleState {
                        name: "Lead".to_string(),
                        color: transparent_color(),
                        z_index: 0,
                    },
                    name: "Alex".to_string(),
                    shortcut: "A".to_string(),
                    color: transparent_color(),
                    icon: None,
                }];
                state.dancers_state.selected_dancer = state.dancers_state.dancers.first().cloned();
                state.dancers_state.can_delete_dancer = true;

                reduce(
                    &mut state,
                    ChoreoMainAction::DancersAction(DancersAction::DeleteSelectedDancer),
                );

                assert!(state.dancers_state.dancers.is_empty());
            },
        );

        spec.it(
            "routes settings content through settings actions and returns to the main page",
            |_| {
                let mut state = ChoreoMainState {
                    content: MainContent::Settings,
                    ..ChoreoMainState::default()
                };

                reduce(
                    &mut state,
                    ChoreoMainAction::SettingsAction(SettingsAction::UpdateAudioPlayerBackend {
                        backend: AudioPlayerBackend::Awedio,
                    }),
                );
                let mut errors = Vec::new();

                check_eq!(
                    errors,
                    state.settings_state.audio_player_backend,
                    AudioPlayerBackend::Awedio
                );

                reduce(
                    &mut state,
                    ChoreoMainAction::SettingsAction(SettingsAction::NavigateBack),
                );
                check_eq!(errors, state.content, MainContent::Main);

                assert_no_errors(errors);
            },
        );
    });

    let report = crate::choreo_main::run_suite(&suite);
    assert!(report.is_success());
}
