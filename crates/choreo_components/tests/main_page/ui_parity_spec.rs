use crate::main_page::Report;
use crate::main_page::actions::ChoreoMainAction;
use crate::main_page::reducer::reduce;
use crate::main_page::state::ChoreoMainState;
use crate::main_page::state::InteractionMode;
use crate::main_page::ui::drawer_host_rect;
use crate::main_page::ui::drawer_host_state;
use crate::main_page::ui::audio_panel_rect;
use crate::main_page::ui::floor_content_rect;
use crate::main_page::ui::home_icon_name;
use crate::main_page::ui::map_choreography_settings_action;
use crate::main_page::ui::map_drawer_host_action;
use crate::main_page::ui::map_scene_pane_action;
use crate::main_page::ui::mode_count;
use crate::main_page::ui::mode_label;
use crate::main_page::ui::nav_icon_name;
use crate::main_page::ui::open_audio_icon_name;
use crate::main_page::ui::open_image_icon_name;
use crate::main_page::ui::scene_pane_state;
use crate::main_page::ui::shell_rect;
use crate::main_page::ui::top_bar_action_count;
use crate::main_page::ui::top_bar_action_icon_tokens;
use crate::main_page::ui::top_bar_action_icon_uris;
use crate::main_page::ui::top_bar_nav_action;
use crate::main_page::ui::top_bar_open_audio_action;
use crate::main_page::ui::top_bar_rect;
use crate::main_page::ui::top_bar_settings_action;
use crate::main_page::ui::top_bar_settings_icon_name;
use crate::main_page::ui::translated_mode_labels;
use choreo_components::choreo_main::actions::OpenAudioRequested;
use choreo_components::choreo_main::actions::OpenChoreoRequested;
use choreo_components::choreography_settings::actions::ChoreographySettingsAction;
use choreo_components::choreography_settings::ui::drawer_width_token as settings_drawer_width_token;
use choreo_components::nav_bar::translations::nav_bar_translations;
use choreo_components::scenes::actions::ScenesAction;
use choreo_master_mobile_json::Color;
use choreo_master_mobile_json::SceneId;
use choreo_models::SceneModel;
use material3::components::drawer_host::actions::DrawerHostAction;
use material3::components::drawer_host::state::DrawerHostOpenMode;
use material3::components::drawer_host::ui::compute_layout as compute_drawer_host_layout;
use material3::components::drawer_host::ui::overlay_visible as drawer_host_overlay_visible;

#[test]
fn ui_parity_spec() {
    let suite = rspec::describe("main page component parity", (), |spec| {
        spec.it("draws without panic for default state", |_| {
            let state = ChoreoMainState::default();
            let context = egui::Context::default();

            let _ = context.run(egui::RawInput::default(), |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    let _ = crate::main_page::ui::draw(ui, &state);
                });
            });
        });

        spec.it("exposes all mode labels", |_| {
            assert_eq!(mode_count(), 6);
            assert_eq!(mode_label(0), "View");
            assert_eq!(mode_label(1), "Move");
            assert_eq!(mode_label(2), "Rotate around center");
            assert_eq!(mode_label(3), "Rotate around dancer");
            assert_eq!(mode_label(4), "Scale");
            assert_eq!(mode_label(5), "Line of sight");
        });

        spec.it(
            "maps top bar toggles and audio open to parity actions",
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
                assert_eq!(
                    top_bar_open_audio_action(),
                    ChoreoMainAction::RequestOpenAudio(OpenAudioRequested {
                        file_path: String::new(),
                        trace_context: None,
                    })
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

        spec.it("keeps the expected top bar action order", |_| {
            assert_eq!(top_bar_action_count(), 6);
            assert_eq!(
                top_bar_action_icon_tokens(false),
                ["menu", "edit", "home", "image", "play_circle"]
            );
            assert_eq!(
                top_bar_action_icon_tokens(true),
                ["close", "edit", "home", "image", "play_circle"]
            );
        });

        spec.it("maps trailing actions to distinct svg sources", |_| {
            assert_eq!(
                top_bar_action_icon_uris(),
                [
                    "bytes://top_bar/settings.svg",
                    "bytes://top_bar/home.svg",
                    "bytes://top_bar/image.svg",
                    "bytes://top_bar/audio.svg",
                ]
            );
        });

        spec.it("exposes all translated mode options", |_| {
            let strings = nav_bar_translations("en");
            assert_eq!(
                translated_mode_labels(&strings),
                [
                    "View",
                    "Move",
                    "Rotate around center",
                    "Rotate around dancer",
                    "Scale",
                    "Line of sight",
                ]
            );
        });

        spec.it(
            "updates mode index and interaction mode from selected menu item",
            |_| {
                let mut state = ChoreoMainState::default();
                reduce(&mut state, ChoreoMainAction::SelectMode { index: 2 });

                assert_eq!(state.selected_mode_index, 2);
                assert_eq!(state.interaction_mode, InteractionMode::RotateAroundCenter);
            },
        );

        spec.it(
            "projects drawer host state below the nav bar with the expected drawer widths",
            |_| {
                let state = ChoreoMainState {
                    is_nav_open: true,
                    is_choreography_settings_open: true,
                    ..ChoreoMainState::default()
                };

                let drawer_state = drawer_host_state(egui::vec2(1280.0, 720.0), &state);

                assert_eq!(drawer_state.left_drawer_width, 324.0);
                assert_eq!(drawer_state.right_drawer_width, settings_drawer_width_token());
                assert_eq!(drawer_state.responsive_breakpoint, 900.0);
                assert_eq!(drawer_state.open_mode, DrawerHostOpenMode::Standard);
                assert_eq!(drawer_state.top_inset, 0.0);
                assert!(!drawer_state.left_close_on_click_away);
                assert!(!drawer_state.right_close_on_click_away);
                assert!(drawer_state.is_left_open);
                assert!(drawer_state.is_right_open);
                assert!(!drawer_host_overlay_visible(&drawer_state, 1280.0));
            },
        );

        spec.it(
            "positions the drawer host directly below the nav bar",
            |_| {
                let page_rect =
                    egui::Rect::from_min_max(egui::pos2(20.0, 30.0), egui::pos2(1300.0, 750.0));

                let top_bar = top_bar_rect(page_rect);
                let drawer_host = drawer_host_rect(page_rect, 0.0);

                assert_eq!(top_bar.top(), 30.0);
                assert_eq!(top_bar.bottom(), 114.0);
                assert_eq!(top_bar.bottom(), drawer_host.top());
                assert_eq!(drawer_host.top(), 114.0);
                assert_eq!(drawer_host.bottom(), 750.0);
                assert_eq!(drawer_host.bottom(), page_rect.bottom());
                assert_eq!(drawer_host.left(), 20.0);
                assert_eq!(drawer_host.right(), 1300.0);
            },
        );

        spec.it(
            "docks the audio player to the bottom and shrinks the drawer host only while visible",
            |_| {
                let page_rect =
                    egui::Rect::from_min_max(egui::pos2(20.0, 30.0), egui::pos2(1300.0, 750.0));

                let drawer_host = drawer_host_rect(page_rect, 84.0);
                let audio_panel = audio_panel_rect(page_rect, 84.0);

                assert_eq!(drawer_host.top(), 114.0);
                assert_eq!(drawer_host.bottom(), 666.0);
                assert_eq!(drawer_host.bottom(), audio_panel.top());
                assert_eq!(audio_panel.top(), 666.0);
                assert_eq!(audio_panel.bottom(), 750.0);
                assert_eq!(audio_panel.bottom(), page_rect.bottom());
            },
        );

        spec.it(
            "anchors horizontal drawer edges to the page and reserves inline left content space",
            |_| {
                let state = ChoreoMainState {
                    is_nav_open: true,
                    is_choreography_settings_open: true,
                    ..ChoreoMainState::default()
                };
                let page_rect =
                    egui::Rect::from_min_max(egui::pos2(20.0, 30.0), egui::pos2(1300.0, 750.0));
                let host_rect = drawer_host_rect(page_rect, 0.0);
                let drawer_state = drawer_host_state(host_rect.size(), &state);
                let layout = compute_drawer_host_layout(host_rect, &drawer_state);

                assert_eq!(layout.left_panel_rect.left(), host_rect.left());
                assert_eq!(layout.left_panel_rect.right(), host_rect.left() + 324.0);
                assert_eq!(layout.right_panel_rect.right(), host_rect.right());
                assert_eq!(
                    layout.right_panel_rect.left(),
                    host_rect.right() - settings_drawer_width_token()
                );
                assert_eq!(layout.content_rect.left(), host_rect.left() + 324.0);
                assert_eq!(layout.content_rect.right(), host_rect.right());

                let floor_rect = floor_content_rect(layout.content_rect, true);
                assert_eq!(floor_rect.left(), host_rect.left() + 324.0);
                assert_eq!(
                    floor_rect.right(),
                    host_rect.right() - settings_drawer_width_token()
                );
            },
        );

        spec.it(
            "anchors shell geometry to the current ui rect instead of the viewport",
            |_| {
                let state = ChoreoMainState {
                    is_nav_open: true,
                    is_choreography_settings_open: true,
                    ..ChoreoMainState::default()
                };
                let context = egui::Context::default();
                let scoped_rect =
                    egui::Rect::from_min_max(egui::pos2(120.0, 96.0), egui::pos2(1320.0, 780.0));
                let mut observed_shell = None;

                let _ = context.run(egui::RawInput::default(), |ctx| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        let _ =
                            ui.scope_builder(egui::UiBuilder::new().max_rect(scoped_rect), |ui| {
                                observed_shell = Some(shell_rect(ui));
                                let _ = crate::main_page::ui::draw(ui, &state);
                            });
                    });
                });

                assert_eq!(observed_shell, Some(scoped_rect));
            },
        );

        spec.it("maps overlay click-away to drawer close actions", |_| {
            let state = ChoreoMainState {
                is_nav_open: true,
                is_choreography_settings_open: true,
                ..ChoreoMainState::default()
            };

            let actions = map_drawer_host_action(
                DrawerHostAction::OverlayClicked {
                    close_left: true,
                    close_right: true,
                    close_top: false,
                    close_bottom: false,
                },
                &state,
            );

            assert_eq!(
                actions,
                vec![ChoreoMainAction::CloseNav, ChoreoMainAction::CloseSettings]
            );
        });

        spec.it(
            "keeps inline left nav open when only the right drawer is eligible for overlay closing",
            |_| {
                let state = ChoreoMainState {
                    is_nav_open: true,
                    is_choreography_settings_open: true,
                    ..ChoreoMainState::default()
                };

                let actions = map_drawer_host_action(
                    DrawerHostAction::OverlayClicked {
                        close_left: false,
                        close_right: true,
                        close_top: false,
                        close_bottom: false,
                    },
                    &state,
                );

                assert_eq!(actions, vec![ChoreoMainAction::CloseSettings]);
            },
        );

        spec.it(
            "maps choreography settings drawer actions into main actions",
            |_| {
                assert_eq!(
                    map_choreography_settings_action(ChoreographySettingsAction::UpdateShowLegend(
                        true
                    )),
                    ChoreoMainAction::ChoreographySettingsAction(
                        ChoreographySettingsAction::UpdateShowLegend(true)
                    )
                );
            },
        );

        spec.it(
            "renders main page with an open choreography settings drawer without panicking",
            |_| {
                let state = ChoreoMainState {
                    is_choreography_settings_open: true,
                    ..ChoreoMainState::default()
                };
                let context = egui::Context::default();

                let output = context.run(egui::RawInput::default(), |ctx| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        let _ = crate::main_page::ui::draw(ui, &state);
                    });
                });
                assert!(!output.shapes.is_empty());
            },
        );

        spec.it(
            "renders main page with an open left drawer without panicking",
            |_| {
                let state = ChoreoMainState {
                    is_nav_open: true,
                    scenes: vec![crate::main_page::state::SceneState {
                        name: "Intro".to_string(),
                        timestamp_seconds: Some(1.0),
                    }],
                    selected_scene_index: Some(0),
                    ..ChoreoMainState::default()
                };
                let context = egui::Context::default();

                let output = context.run(egui::RawInput::default(), |ctx| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        let _ = crate::main_page::ui::draw(ui, &state);
                    });
                });
                assert!(!output.shapes.is_empty());
            },
        );

        spec.it(
            "projects the shared scenes pane state for the left drawer",
            |_| {
                let mut state = ChoreoMainState {
                    selected_scene_index: Some(1),
                    ..ChoreoMainState::default()
                };
                state.scene_search_text = "sec".to_string();
                state.choreography_settings_state.show_timestamps = true;
                state.choreography_settings_state.choreography.scenes = vec![
                    SceneModel {
                        scene_id: SceneId(1),
                        positions: Vec::new(),
                        name: "Intro".to_string(),
                        text: Some("opening".to_string()),
                        fixed_positions: false,
                        timestamp: Some("1.0".to_string()),
                        variation_depth: 0,
                        variations: Vec::new(),
                        current_variation: Vec::new(),
                        color: Color::transparent(),
                    },
                    SceneModel {
                        scene_id: SceneId(2),
                        positions: Vec::new(),
                        name: "Second".to_string(),
                        text: Some("middle".to_string()),
                        fixed_positions: false,
                        timestamp: Some("2.5".to_string()),
                        variation_depth: 0,
                        variations: Vec::new(),
                        current_variation: Vec::new(),
                        color: Color::transparent(),
                    },
                ];

                let pane_state = scene_pane_state(&state);

                assert_eq!(pane_state.search_text, "sec");
                assert!(pane_state.show_timestamps);
                assert_eq!(pane_state.scenes.len(), 2);
                assert_eq!(pane_state.visible_scenes.len(), 1);
                assert_eq!(pane_state.visible_scenes[0].name, "Second");
                assert!(pane_state.visible_scenes[0].is_selected);
                assert!(pane_state.choreography.scenes.is_empty());
                assert!(pane_state.scenes[0].positions.is_empty());
                assert!(pane_state.scenes[0].variations.is_empty());
                assert!(pane_state.scenes[0].current_variation.is_empty());
                assert!(!pane_state.can_save_choreo);
                assert!(pane_state.can_navigate_to_settings);
                assert!(pane_state.can_navigate_to_dancer_settings);
            },
        );

        spec.it(
            "enables shared scenes save when a choreography file path exists",
            |_| {
                let temp_root = std::env::temp_dir().join("rchoreo_main_page_save_enablement_spec");
                std::fs::create_dir_all(&temp_root).expect("temp dir should be created");
                let file_path = temp_root.join("demo.choreo");
                std::fs::write(&file_path, "{}").expect("temp file should be created");

                let mut state = ChoreoMainState {
                    ..ChoreoMainState::default()
                };
                state.last_opened_choreo_file = Some(file_path.to_string_lossy().into_owned());
                state.choreography_settings_state.choreography.name = "Demo".to_string();

                let pane_state = scene_pane_state(&state);

                assert!(pane_state.can_save_choreo);

                let _ = std::fs::remove_file(file_path);
                let _ = std::fs::remove_dir(temp_root);
            },
        );

        spec.it(
            "maps shared scenes pane actions into main page actions",
            |_| {
                assert_eq!(
                    map_scene_pane_action(ScenesAction::SelectScene { index: 2 }),
                    Some(ChoreoMainAction::SelectScene { index: 2 })
                );
                assert_eq!(
                    map_scene_pane_action(ScenesAction::NavigateToSettings),
                    Some(ChoreoMainAction::NavigateToSettings)
                );
                assert_eq!(
                    map_scene_pane_action(ScenesAction::NavigateToDancerSettings),
                    Some(ChoreoMainAction::NavigateToDancers)
                );
                assert_eq!(
                    map_scene_pane_action(ScenesAction::RequestOpenChoreography),
                    Some(ChoreoMainAction::RequestOpenChoreo(OpenChoreoRequested {
                        file_path: None,
                        file_name: None,
                        contents: String::new(),
                    }))
                );
                assert_eq!(
                    map_scene_pane_action(ScenesAction::RequestSaveChoreography),
                    Some(ChoreoMainAction::RequestSaveChoreo)
                );
                assert_eq!(
                    map_scene_pane_action(ScenesAction::InsertScene {
                        insert_after: false
                    }),
                    Some(ChoreoMainAction::InsertScene {
                        insert_after: false
                    })
                );
                assert_eq!(
                    map_scene_pane_action(ScenesAction::OpenDeleteSceneDialog),
                    Some(ChoreoMainAction::DeleteSelectedScene)
                );
            },
        );
    });

    let report = crate::main_page::run_suite(&suite);
    assert!(report.is_success());
}
