use crate::choreo_main::Report;
use crate::choreo_main::actions::ChoreoMainAction;
use crate::choreo_main::reducer::reduce;
use crate::choreo_main::state::ChoreoMainState;
use crate::choreo_main::state::InteractionMode;
use crate::choreo_main::ui::mode_count;
use crate::choreo_main::ui::mode_label;

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
    });

    let report = crate::choreo_main::run_suite(&suite);
    assert!(report.is_success());
}
