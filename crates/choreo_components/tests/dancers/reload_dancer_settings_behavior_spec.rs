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
fn reload_dancer_settings_behavior_spec() {
    let suite = rspec::describe("reload dancer settings behavior", (), |spec| {
        spec.it("restores local edits from global state snapshot", |_| {
            let role = dancers::role("Lead");
            let mut state = dancers::state::DancersState::default().with_global(
                dancers::state::DancersGlobalState {
                    roles: vec![role.clone()],
                    dancers: vec![dancers::dancer(1, role, "Alice", "A", None)],
                    ..dancers::state::DancersGlobalState::default()
                },
            );

            dancers::reducer::reduce(&mut state, dancers::actions::DancersAction::LoadFromGlobal);
            dancers::reducer::reduce(
                &mut state,
                dancers::actions::DancersAction::UpdateDancerName {
                    value: "Edited Locally".to_string(),
                },
            );
            let mut errors = Vec::new();

            check_eq!(errors, state.dancers[0].name.as_str(), "Edited Locally");

            dancers::reducer::reduce(
                &mut state,
                dancers::actions::DancersAction::ReloadFromGlobal,
            );
            check_eq!(errors, state.dancers[0].name.as_str(), "Alice");

            assert_no_errors(errors);
        });
    });
    let report = dancers::run_suite(&suite);
    assert!(report.is_success());
}
