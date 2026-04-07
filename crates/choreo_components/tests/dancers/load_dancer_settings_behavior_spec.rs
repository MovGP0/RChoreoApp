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
fn load_dancer_settings_behavior_spec() {
    let suite = rspec::describe("load dancer settings behavior", (), |spec| {
        spec.it("loads roles and dancers from global state", |_| {
            let role_a = dancers::role("Lead");
            let role_b = dancers::role("Follow");
            let dancer_a = dancers::dancer(1, role_a.clone(), "Alice", "A", None);
            let dancer_b = dancers::dancer(2, role_b, "Bob", "B", None);

            let mut state = dancers::state::DancersState::default().with_global(
                dancers::state::DancersGlobalState {
                    roles: vec![role_a, dancers::role("Follow")],
                    dancers: vec![dancer_a, dancer_b],
                    ..dancers::state::DancersGlobalState::default()
                },
            );

            dancers::reducer::reduce(&mut state, dancers::actions::DancersAction::LoadFromGlobal);

            let mut errors = Vec::new();

            check_eq!(errors, state.roles.len(), 2);
            check_eq!(errors, state.roles[0].name, "Lead");
            check_eq!(errors, state.dancers.len(), 2);
            check_eq!(errors, state.dancers[0].name, "Alice");
            check_eq!(
                errors,
                state
                    .selected_dancer
                    .as_ref()
                    .map(|dancer| dancer.dancer_id),
                Some(1)
            );

            assert_no_errors(errors);
        });
    });
    let report = dancers::run_suite(&suite);
    assert!(report.is_success());
}
