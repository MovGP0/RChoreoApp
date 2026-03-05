use crate::main_page::Report;
use crate::main_page::actions::ChoreoMainAction;
use crate::main_page::reducer::reduce;
use crate::main_page::state::ChoreoMainState;
use crate::main_page::state::InteractionMode;
use crate::main_page::ui::drawer_host_state;
use crate::main_page::ui::home_icon_name;
use crate::main_page::ui::map_choreography_settings_action;
use crate::main_page::ui::map_drawer_host_action;
use crate::main_page::ui::mode_count;
use crate::main_page::ui::mode_label;
use crate::main_page::ui::nav_icon_name;
use crate::main_page::ui::open_audio_icon_name;
use crate::main_page::ui::open_image_icon_name;
use crate::main_page::ui::top_bar_nav_action;
use crate::main_page::ui::top_bar_settings_action;
use crate::main_page::ui::top_bar_settings_icon_name;
use choreo_components_egui::choreography_settings::actions::ChoreographySettingsAction;
use choreo_components_egui::main_page_drawer_host::actions::MainPageDrawerHostAction;

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
            "updates mode index and interaction mode from selected menu item",
            |_| {
                let mut state = ChoreoMainState::default();
                reduce(&mut state, ChoreoMainAction::SelectMode { index: 2 });

                assert_eq!(state.selected_mode_index, 2);
                assert_eq!(state.interaction_mode, InteractionMode::RotateAroundCenter);
            },
        );

        spec.it(
            "projects drawer host state with parity insets and drawer widths",
            |_| {
                let state = ChoreoMainState {
                    is_nav_open: true,
                    is_choreography_settings_open: true,
                    ..ChoreoMainState::default()
                };

                let drawer_state = drawer_host_state(egui::vec2(1280.0, 720.0), &state);

                assert_eq!(drawer_state.left_drawer_width, 324.0);
                assert_eq!(drawer_state.right_drawer_width, 480.0);
                assert_eq!(drawer_state.top_inset, 84.0);
                assert!(drawer_state.is_left_open);
                assert!(drawer_state.is_right_open);
            },
        );

        spec.it("maps overlay click-away to drawer close actions", |_| {
            let state = ChoreoMainState {
                is_nav_open: true,
                is_choreography_settings_open: true,
                ..ChoreoMainState::default()
            };

            let actions = map_drawer_host_action(MainPageDrawerHostAction::OverlayClicked, &state);

            assert_eq!(
                actions,
                vec![ChoreoMainAction::CloseNav, ChoreoMainAction::CloseSettings]
            );
        });

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
            "renders choreography settings content inside the right drawer",
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

                let mut found_heading = false;
                for clipped in output.shapes {
                    if format!("{:?}", clipped.shape).contains("Choreography Settings") {
                        found_heading = true;
                        break;
                    }
                }

                assert!(found_heading);
            },
        );
    });

    let report = crate::main_page::run_suite(&suite);
    assert!(report.is_success());
}
