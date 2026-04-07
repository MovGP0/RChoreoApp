use crate::choreo_main::Report;
use crate::choreo_main::actions::ChoreoMainAction;
use crate::choreo_main::reducer::reduce;
use crate::choreo_main::state::ChoreoMainState;

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
fn hide_dialog_behavior_spec() {
    let suite = rspec::describe("hide dialog reducer behavior", (), |spec| {
        spec.it("closes dialog when hide action is dispatched", |_| {
            let mut state = ChoreoMainState::default();
            reduce(
                &mut state,
                ChoreoMainAction::ShowDialog {
                    content: Some("hello".to_string()),
                },
            );
            reduce(&mut state, ChoreoMainAction::HideDialog);

            let mut errors = Vec::new();

            check!(errors, !state.is_dialog_open);
            check!(errors, state.dialog_content.is_none());

            assert_no_errors(errors);
        });
    });

    let report = crate::choreo_main::run_suite(&suite);
    assert!(report.is_success());
}
