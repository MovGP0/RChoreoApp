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
fn delete_dancer_behavior_spec() {
    let suite = rspec::describe("delete dancer behavior", (), |spec| {
        spec.it("deletes selected dancer and keeps selection valid", |_| {
            let role = dancers::role("Role");
            let mut state = dancers::state::DancersState::default().with_global(
                dancers::state::DancersGlobalState {
                    dancers: vec![
                        dancers::dancer(1, role.clone(), "A", "A", None),
                        dancers::dancer(2, role, "B", "B", None),
                    ],
                    ..dancers::state::DancersGlobalState::default()
                },
            );

            dancers::reducer::reduce(&mut state, dancers::actions::DancersAction::LoadFromGlobal);
            dancers::reducer::reduce(
                &mut state,
                dancers::actions::DancersAction::DeleteSelectedDancer,
            );

            let mut errors = Vec::new();

            check_eq!(errors, state.dancers.len(), 1);
            check_eq!(errors, state.dancers[0].dancer_id, 2);
            check_eq!(
                errors,
                state
                    .selected_dancer
                    .as_ref()
                    .map(|dancer| dancer.dancer_id),
                Some(2)
            );

            dancers::reducer::reduce(
                &mut state,
                dancers::actions::DancersAction::DeleteSelectedDancer,
            );

            check!(errors, state.dancers.is_empty());
            check!(errors, state.selected_dancer.is_none());

            assert_no_errors(errors);
        });
    });
    let report = dancers::run_suite(&suite);
    assert!(report.is_success());
}
