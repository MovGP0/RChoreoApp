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
fn selected_role_behavior_spec() {
    let suite = rspec::describe("selected role behavior", (), |spec| {
        spec.it("updates selected dancer role by role index", |_| {
            let lead = dancers::role("Lead");
            let follow = dancers::role("Follow");
            let mut state = dancers::state::DancersState::default().with_global(
                dancers::state::DancersGlobalState {
                    roles: vec![lead.clone(), follow.clone()],
                    dancers: vec![dancers::dancer(1, lead, "A", "A", None)],
                    ..dancers::state::DancersGlobalState::default()
                },
            );

            dancers::reducer::reduce(&mut state, dancers::actions::DancersAction::LoadFromGlobal);
            dancers::reducer::reduce(
                &mut state,
                dancers::actions::DancersAction::SelectRole { index: 1 },
            );

            let mut errors = Vec::new();

            check_eq!(
                errors,
                state
                    .selected_dancer
                    .as_ref()
                    .map(|dancer| dancer.role.name.as_str()),
                Some("Follow")
            );
            check_eq!(errors, state.dancers[0].role.name.as_str(), "Follow");
            check_eq!(
                errors,
                state.selected_role.as_ref().map(|role| role.name.as_str()),
                Some("Follow")
            );

            assert_no_errors(errors);
        });
    });
    let report = dancers::run_suite(&suite);
    assert!(report.is_success());
}
