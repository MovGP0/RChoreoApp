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
fn add_dancer_behavior_spec() {
    let suite = rspec::describe("add dancer behavior", (), |spec| {
        spec.it("adds the next dancer and selects it", |_| {
            let lead = dancers::role("Lead");
            let mut state = dancers::state::DancersState::default().with_global(
                dancers::state::DancersGlobalState {
                    roles: vec![lead.clone()],
                    dancers: vec![dancers::dancer(1, lead, "A", "A", None)],
                    ..dancers::state::DancersGlobalState::default()
                },
            );

            dancers::reducer::reduce(&mut state, dancers::actions::DancersAction::LoadFromGlobal);
            dancers::reducer::reduce(&mut state, dancers::actions::DancersAction::AddDancer);

            let mut errors = Vec::new();

            check_eq!(errors, state.dancers.len(), 2);
            let selected = state
                .selected_dancer
                .as_ref()
                .expect("new dancer should be selected");
            check_eq!(errors, selected.dancer_id, 2);
            check!(errors, selected.name.is_empty());
            check_eq!(errors, selected.role.name, "Lead");

            assert_no_errors(errors);
        });
    });
    let report = dancers::run_suite(&suite);
    assert!(report.is_success());
}
