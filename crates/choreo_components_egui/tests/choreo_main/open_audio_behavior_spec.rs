use crate::choreo_main::Report;
use crate::choreo_main::actions::ChoreoMainAction;
use crate::choreo_main::actions::OpenAudioRequested;
use crate::choreo_main::reducer::reduce;
use crate::choreo_main::state::ChoreoMainState;

#[test]
fn open_audio_behavior_spec() {
    let suite = rspec::describe("open audio reducer behavior", (), |spec| {
        spec.it("forwards requested file path to outgoing audio request queue", |_| {
            let mut state = ChoreoMainState::default();
            let request = OpenAudioRequested {
                file_path: "C:/music.mp3".to_string(),
                trace_context: None,
            };
            reduce(
                &mut state,
                ChoreoMainAction::RequestOpenAudio(request.clone()),
            );

            assert_eq!(state.outgoing_audio_requests.len(), 1);
            assert_eq!(state.outgoing_audio_requests[0], request);
        });
    });

    let report = crate::choreo_main::run_suite(&suite);
    assert!(report.is_success());
}
