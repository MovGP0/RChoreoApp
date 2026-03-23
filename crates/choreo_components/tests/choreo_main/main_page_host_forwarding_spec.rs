use crate::choreo_main::Report;
use crate::choreo_main::actions::ChoreoMainAction;
use crate::choreo_main::state::ChoreoMainState;
use crate::choreo_main::ui::map_audio_host_action;
use choreo_components::audio_player::actions::AudioPlayerAction;
use choreo_components::floor::actions::FloorAction;
use std::cell::RefCell;
use std::rc::Rc;

#[test]
fn main_page_host_forwarding_spec() {
    let suite = rspec::describe("main page host forwarding", (), |spec| {
        spec.it(
            "forwards floor host draw actions to the choreo_main action pipeline",
            |_| {
                let state = ChoreoMainState::default();
                let context = egui::Context::default();
                let captured: Rc<RefCell<Vec<ChoreoMainAction>>> =
                    Rc::new(RefCell::new(Vec::new()));
                let captured_for_draw = Rc::clone(&captured);

                let _ = context.run(egui::RawInput::default(), |ctx| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        let actions = crate::choreo_main::ui::draw(ui, &state);
                        captured_for_draw.borrow_mut().extend(actions);
                    });
                });

                let actions = captured.borrow();
                assert!(actions.iter().any(|action| {
                    matches!(
                        action,
                        ChoreoMainAction::FloorAction(FloorAction::SetLayout { .. })
                    )
                }));
                assert!(actions.iter().any(|action| {
                    matches!(
                        action,
                        ChoreoMainAction::FloorAction(FloorAction::DrawFloor)
                    )
                }));
            },
        );

        spec.it(
            "maps audio seek and scene-link actions to choreo_main actions",
            |_| {
                let seek_actions =
                    map_audio_host_action(AudioPlayerAction::SeekToPosition { position: 12.3 });
                assert_eq!(
                    seek_actions,
                    vec![
                        ChoreoMainAction::AudioPlayerAction(AudioPlayerAction::SeekToPosition {
                            position: 12.3
                        }),
                        ChoreoMainAction::UpdateAudioPosition { seconds: 12.3 }
                    ]
                );

                let link_actions = map_audio_host_action(AudioPlayerAction::LinkSceneToPosition);
                assert_eq!(
                    link_actions,
                    vec![
                        ChoreoMainAction::AudioPlayerAction(AudioPlayerAction::LinkSceneToPosition),
                        ChoreoMainAction::LinkSelectedSceneToAudioPosition
                    ]
                );
            },
        );
    });

    let report = crate::choreo_main::run_suite(&suite);
    assert!(report.is_success());
}
