use crate::dancers;
use dancers::Report;

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
fn hide_dancer_dialog_behavior_spec() {
    let suite = rspec::describe("hide dancer dialog behavior", (), |spec| {
        spec.it("clears dialog state", |_| {
            let mut state = dancers::state::DancersState::default();
            let mut errors = Vec::new();

            dancers::reducer::reduce(
                &mut state,
                dancers::actions::DancersAction::ShowDialog {
                    content_id: Some("swap_dancers".to_string()),
                },
            );
            check!(errors, state.is_dialog_open);

            dancers::reducer::reduce(&mut state, dancers::actions::DancersAction::HideDialog);
            check!(errors, !state.is_dialog_open);
            check!(errors, state.dialog_content.is_none());

            assert_no_errors(errors);
        });
    });
    let report = dancers::run_suite(&suite);
    assert!(report.is_success());
}
