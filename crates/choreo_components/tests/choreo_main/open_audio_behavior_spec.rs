use crate::choreo_main::Report;
use crate::choreo_main::actions::ChoreoMainAction;
use crate::choreo_main::actions::OpenAudioRequested;
use crate::choreo_main::reducer::reduce;
use crate::choreo_main::state::ChoreoMainState;

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

fn assert_no_errors(errors: Vec<String>) {
    assert!(
        errors.is_empty(),
        "Assertion failures:\n{}",
        errors.join("\n")
    );
}

#[test]
fn open_audio_behavior_spec() {
    let suite = rspec::describe("open audio reducer behavior", (), |spec| {
        spec.it(
            "forwards requested file path to outgoing audio request queue",
            |_| {
                let mut state = ChoreoMainState::default();
                let request = OpenAudioRequested {
                    file_path: "C:/music.mp3".to_string(),
                    trace_context: None,
                };
                reduce(
                    &mut state,
                    ChoreoMainAction::RequestOpenAudio(request.clone()),
                );

                let mut errors = Vec::new();

                check_eq!(errors, state.outgoing_audio_requests, vec![request]);

                assert_no_errors(errors);
            },
        );
    });

    let report = crate::choreo_main::run_suite(&suite);
    assert!(report.is_success());
}
