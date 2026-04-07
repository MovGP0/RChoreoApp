use crate::choreo_main::Report;
use crate::choreo_main::actions::ChoreoMainAction;
use crate::choreo_main::actions::OpenAudioRequested;
use crate::choreo_main::actions::OpenSvgFileCommand;
use crate::choreo_main::reducer::reduce;
use crate::choreo_main::reducer::reduce_with_behaviors;
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
fn open_svg_file_behavior_spec() {
    let suite = rspec::describe("open svg file behavior", (), |spec| {
        spec.it(
            "opens svg path, stores preference, and requests draw",
            |_| {
                let mut state = ChoreoMainState::default();
                reduce_with_behaviors(
                    &mut state,
                    ChoreoMainAction::ApplyOpenSvgFile(OpenSvgFileCommand {
                        file_path: "C:/image.svg".to_string(),
                    }),
                );

                let mut errors = Vec::new();

                check_eq!(errors, state.svg_file_path.as_deref(), Some("C:/image.svg"));
                check_eq!(
                    errors,
                    state.last_opened_svg_preference.as_deref(),
                    Some("C:/image.svg")
                );
                check_eq!(errors, state.draw_floor_request_count, 1);

                assert_no_errors(errors);
            },
        );

        spec.it(
            "restores existing svg path and clears queued outgoing commands",
            |_| {
                let mut state = ChoreoMainState::default();
                reduce(
                    &mut state,
                    ChoreoMainAction::RestoreLastOpenedSvg {
                        file_path: Some("C:/restored.svg".to_string()),
                        path_exists: true,
                    },
                );
                let mut errors = Vec::new();

                check_eq!(errors, state.svg_file_path.as_deref(), Some("C:/restored.svg"));
                check_eq!(
                    errors,
                    state.last_opened_svg_preference.as_deref(),
                    Some("C:/restored.svg")
                );
                check_eq!(errors, state.draw_floor_request_count, 1);

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
                check_eq!(errors, state.outgoing_open_svg_commands.len(), 1);
                check_eq!(errors, state.outgoing_audio_requests.len(), 1);

                reduce(&mut state, ChoreoMainAction::ClearOutgoingCommands);
                check!(errors, state.outgoing_open_svg_commands.is_empty());
                check!(errors, state.outgoing_audio_requests.is_empty());

                assert_no_errors(errors);
            },
        );
    });

    let report = crate::choreo_main::run_suite(&suite);
    assert!(report.is_success());
}
