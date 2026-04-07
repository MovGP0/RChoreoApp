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
fn show_dialog_behavior_spec() {
    let suite = rspec::describe("show dialog reducer behavior", (), |spec| {
        spec.it("opens dialog with provided content", |_| {
            let mut state = ChoreoMainState::default();
            reduce(
                &mut state,
                ChoreoMainAction::ShowDialog {
                    content: Some("dialog content".to_string()),
                },
            );

            let mut errors = Vec::new();

            check_eq!(errors, state.is_dialog_open, true);
            check_eq!(
                errors,
                state.dialog_content.as_deref(),
                Some("dialog content")
            );

            assert_no_errors(errors);
        });
    });

    let report = crate::choreo_main::run_suite(&suite);
    assert!(report.is_success());
}
