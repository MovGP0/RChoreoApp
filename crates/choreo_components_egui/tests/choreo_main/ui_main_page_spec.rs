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
use crate::choreo_main::ui::top_bar_settings_action;
use crate::choreo_main::ui::top_bar_settings_icon_name;
use choreo_components_egui::choreography_settings::actions::ChoreographySettingsAction;
use choreo_components_egui::choreography_settings::actions::UpdateSelectedSceneAction;
use choreo_components_egui::dancers::actions::DancersAction;
use choreo_components_egui::dancers::state::DancerState;
use choreo_components_egui::dancers::state::RoleState;
use choreo_components_egui::dancers::state::transparent_color;
use choreo_components_egui::settings::actions::SettingsAction;
use choreo_components_egui::settings::state::AudioPlayerBackend;

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
            assert_eq!(mode_count(), 6);
            assert_eq!(mode_label(0), "View");
            assert_eq!(mode_label(1), "Move");
            assert_eq!(mode_label(2), "Rotate around center");
            assert_eq!(mode_label(3), "Rotate around dancer");
            assert_eq!(mode_label(4), "Scale");
            assert_eq!(mode_label(5), "Line of sight");
        });

        spec.it(
            "maps top bar toggles to nav/settings open-close actions",
            |_| {
                assert_eq!(top_bar_nav_action(false), ChoreoMainAction::ToggleNav);
                assert_eq!(top_bar_nav_action(true), ChoreoMainAction::CloseNav);
                assert_eq!(
                    top_bar_settings_action(false),
                    ChoreoMainAction::OpenSettings
                );
                assert_eq!(
                    top_bar_settings_action(true),
                    ChoreoMainAction::CloseSettings
                );
            },
        );

        spec.it("uses parity icon tokens for top bar actions", |_| {
            assert_eq!(nav_icon_name(false), "menu");
            assert_eq!(nav_icon_name(true), "close");
            assert_eq!(top_bar_settings_icon_name(), "edit");
            assert_eq!(home_icon_name(), "home");
            assert_eq!(open_image_icon_name(), "image");
            assert_eq!(open_audio_icon_name(), "play_circle");
        });

        spec.it(
            "updates mode index and interaction mode from a selected menu item",
            |_| {
                let mut state = ChoreoMainState::default();
                reduce(&mut state, ChoreoMainAction::SelectMode { index: 2 });

                assert_eq!(state.selected_mode_index, 2);
                assert_eq!(state.interaction_mode, InteractionMode::RotateAroundCenter);
            },
        );

        spec.it(
            "tracks drawer and audio panel open and close actions",
            |_| {
                let mut state = ChoreoMainState::default();
                reduce(&mut state, ChoreoMainAction::ToggleNav);
                reduce(&mut state, ChoreoMainAction::OpenSettings);
                reduce(&mut state, ChoreoMainAction::OpenAudioPanel);

                assert!(state.is_nav_open);
                assert!(state.is_choreography_settings_open);
                assert!(state.is_audio_player_open);

                reduce(&mut state, ChoreoMainAction::CloseNav);
                reduce(&mut state, ChoreoMainAction::CloseSettings);
                reduce(&mut state, ChoreoMainAction::CloseAudioPanel);

                assert!(!state.is_nav_open);
                assert!(!state.is_choreography_settings_open);
                assert!(!state.is_audio_player_open);
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

                assert_eq!(state.choreography_settings_state.scene_name, "Finale");
                assert!(state.choreography_settings_state.scene_has_timestamp);
                assert!(
                    (state.choreography_settings_state.scene_timestamp_seconds - 3.0).abs()
                        < 0.0001
                );
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

                assert_eq!(state.choreography_settings_state.scene_name, "Updated");
                assert_eq!(state.scenes[0].name, "Updated");
                assert_eq!(state.floor_scene_name.as_deref(), Some("Updated"));
                assert_eq!(state.draw_floor_request_count, 1);
            },
        );

        spec.it(
            "routes dancers content through dancers pane and dispatches dancer actions",
            |_| {
                let mut state = ChoreoMainState::default();
                state.content = MainContent::Dancers;
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
                let mut state = ChoreoMainState::default();
                state.content = MainContent::Settings;

                reduce(
                    &mut state,
                    ChoreoMainAction::SettingsAction(SettingsAction::UpdateAudioPlayerBackend {
                        backend: AudioPlayerBackend::Awedio,
                    }),
                );
                assert_eq!(
                    state.settings_state.audio_player_backend,
                    AudioPlayerBackend::Awedio
                );

                reduce(
                    &mut state,
                    ChoreoMainAction::SettingsAction(SettingsAction::NavigateBack),
                );
                assert_eq!(state.content, MainContent::Main);
            },
        );
    });

    let report = crate::choreo_main::run_suite(&suite);
    assert!(report.is_success());
}
