use crate::dancers;
use dancers::Report;

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
fn cancel_dancer_settings_behavior_spec() {
    let suite = rspec::describe("cancel dancer settings behavior", (), |spec| {
        spec.it("does not mutate dialog state", |_| {
            let mut state = dancers::state::DancersState {
                is_dialog_open: true,
                dialog_content: Some("swap_dancers".to_string()),
                ..dancers::state::DancersState::default()
            };

            dancers::reducer::reduce(&mut state, dancers::actions::DancersAction::Cancel);

            let mut errors = Vec::new();

            check_eq!(errors, state.is_dialog_open, true);
            check_eq!(
                errors,
                state.dialog_content.as_deref(),
                Some("swap_dancers")
            );

            assert_no_errors(errors);
        });
    });
    let report = dancers::run_suite(&suite);
    assert!(report.is_success());
}
