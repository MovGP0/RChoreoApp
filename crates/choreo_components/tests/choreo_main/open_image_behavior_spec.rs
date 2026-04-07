use crate::choreo_main::Report;
use crate::choreo_main::actions::ChoreoMainAction;
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
fn open_image_behavior_spec() {
    let suite = rspec::describe("open image reducer behavior", (), |spec| {
        spec.it("maps image request into open-svg command", |_| {
            let mut state = ChoreoMainState::default();
            reduce(
                &mut state,
                ChoreoMainAction::RequestOpenImage {
                    file_path: "C:/image.svg".to_string(),
                },
            );

            let mut errors = Vec::new();

            check_eq!(errors, state.outgoing_open_svg_commands.len(), 1);
            check_eq!(
                errors,
                state.outgoing_open_svg_commands[0].file_path,
                "C:/image.svg"
            );

            assert_no_errors(errors);
        });
    });

    let report = crate::choreo_main::run_suite(&suite);
    assert!(report.is_success());
}
