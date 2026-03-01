use crate::choreo_main::Report;
use crate::choreo_main::actions::ChoreoMainAction;
use crate::choreo_main::actions::OpenAudioRequested;
use crate::choreo_main::actions::OpenSvgFileCommand;
use crate::choreo_main::reducer::reduce;
use crate::choreo_main::state::ChoreoMainState;

#[test]
fn open_svg_file_behavior_spec() {
    let suite = rspec::describe("open svg file behavior", (), |spec| {
        spec.it("opens svg path, stores preference, and requests draw", |_| {
            let mut state = ChoreoMainState::default();
            reduce(
                &mut state,
                ChoreoMainAction::ApplyOpenSvgFile(OpenSvgFileCommand {
                    file_path: "C:/image.svg".to_string(),
                }),
            );

            assert_eq!(state.svg_file_path.as_deref(), Some("C:/image.svg"));
            assert_eq!(
                state.last_opened_svg_preference.as_deref(),
                Some("C:/image.svg")
            );
            assert_eq!(state.draw_floor_request_count, 1);
        });

        spec.it("restores existing svg path and clears queued outgoing commands", |_| {
            let mut state = ChoreoMainState::default();
            reduce(
                &mut state,
                ChoreoMainAction::RestoreLastOpenedSvg {
                    file_path: Some("C:/restored.svg".to_string()),
                    path_exists: true,
                },
            );
            assert_eq!(state.svg_file_path.as_deref(), Some("C:/restored.svg"));
            assert_eq!(
                state.last_opened_svg_preference.as_deref(),
                Some("C:/restored.svg")
            );
            assert_eq!(state.draw_floor_request_count, 1);

            reduce(
                &mut state,
                ChoreoMainAction::RequestOpenImage {
                    file_path: "C:/queued.svg".to_string(),
                },
            );
            reduce(
                &mut state,
                ChoreoMainAction::RequestOpenAudio(OpenAudioRequested {
                    file_path: "C:/queued.mp3".to_string(),
                    trace_context: None,
                }),
            );
            assert_eq!(state.outgoing_open_svg_commands.len(), 1);
            assert_eq!(state.outgoing_audio_requests.len(), 1);

            reduce(&mut state, ChoreoMainAction::ClearOutgoingCommands);
            assert!(state.outgoing_open_svg_commands.is_empty());
            assert!(state.outgoing_audio_requests.is_empty());
        });
    });

    let report = crate::choreo_main::run_suite(&suite);
    assert!(report.is_success());
}
