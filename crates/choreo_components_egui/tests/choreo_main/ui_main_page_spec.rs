use crate::choreo_main::Report;
use crate::choreo_main::actions::ChoreoMainAction;
use crate::choreo_main::reducer::reduce;
use crate::choreo_main::state::ChoreoMainState;
use crate::choreo_main::state::InteractionMode;
use crate::choreo_main::ui::home_icon_name;
use crate::choreo_main::ui::mode_count;
use crate::choreo_main::ui::mode_label;
use crate::choreo_main::ui::nav_icon_name;
use crate::choreo_main::ui::open_audio_icon_name;
use crate::choreo_main::ui::open_image_icon_name;
use crate::choreo_main::ui::top_bar_nav_action;
use crate::choreo_main::ui::top_bar_settings_action;
use crate::choreo_main::ui::top_bar_settings_icon_name;
use choreo_components_egui::dancers::actions::DancersAction;
use choreo_components_egui::dancers::state::DancerState;
use choreo_components_egui::dancers::state::RoleState;
use choreo_components_egui::dancers::state::transparent_color;

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
            assert_eq!(mode_label(2), "Rotate Center");
            assert_eq!(mode_label(3), "Rotate Dancer");
            assert_eq!(mode_label(4), "Scale");
            assert_eq!(mode_label(5), "Line of Sight");
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
            "routes dancers content through dancers pane and dispatches dancer actions",
            |_| {
                let mut state = ChoreoMainState::default();
                state.content = crate::choreo_main::state::MainContent::Dancers;
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
    });

    let report = crate::choreo_main::run_suite(&suite);
    assert!(report.is_success());
}
