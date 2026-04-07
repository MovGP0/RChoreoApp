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
fn swap_dancers_behavior_spec() {
    let suite = rspec::describe("swap dancers behavior", (), |spec| {
        spec.it(
            "swaps role name shortcut color and icon while keeping dancer identities",
            |_| {
                let mut errors = Vec::new();

                let gentleman = dancers::role("Gentleman");
                let lady = dancers::role("Lady");
                let mut first = dancers::dancer(1, gentleman.clone(), "Alex", "AL", Some("IconA"));
                first.color = dancers::color(10, 20, 30);
                let mut second = dancers::dancer(2, lady.clone(), "Bella", "BE", Some("IconB"));
                second.color = dancers::color(40, 50, 60);

                let mut state = dancers::state::DancersState::default().with_global(
                    dancers::state::DancersGlobalState {
                        roles: vec![gentleman, lady],
                        dancers: vec![first, second],
                        ..dancers::state::DancersGlobalState::default()
                    },
                );

                dancers::reducer::reduce(
                    &mut state,
                    dancers::actions::DancersAction::LoadFromGlobal,
                );
                dancers::reducer::reduce(
                    &mut state,
                    dancers::actions::DancersAction::RequestSwapDancers,
                );

                check!(errors, state.is_dialog_open);
                check_eq!(errors, state.dialog_content.as_deref(), Some("swap_dancers"));

                dancers::reducer::reduce(
                    &mut state,
                    dancers::actions::DancersAction::ConfirmSwapDancers,
                );

                check_eq!(errors, state.dancers[0].dancer_id, 1);
                check_eq!(errors, state.dancers[1].dancer_id, 2);
                check_eq!(errors, state.dancers[0].role.name, "Lady");
                check_eq!(errors, state.dancers[1].role.name, "Gentleman");
                check_eq!(errors, state.dancers[0].name, "Bella");
                check_eq!(errors, state.dancers[1].name, "Alex");
                check_eq!(errors, state.dancers[0].shortcut, "BE");
                check_eq!(errors, state.dancers[1].shortcut, "AL");
                check_eq!(errors, state.dancers[0].color.r, 40);
                check_eq!(errors, state.dancers[1].color.r, 10);
                check_eq!(errors, state.dancers[0].icon.as_deref(), Some("IconB"));
                check_eq!(errors, state.dancers[1].icon.as_deref(), Some("IconA"));

                assert_no_errors(errors);
            },
        );

        spec.it("does not swap when selection is invalid", |_| {
            let mut errors = Vec::new();

            let role = dancers::role("Role");
            let mut state = dancers::state::DancersState::default().with_global(
                dancers::state::DancersGlobalState {
                    dancers: vec![dancers::dancer(1, role, "A", "A", None)],
                    ..dancers::state::DancersGlobalState::default()
                },
            );

            dancers::reducer::reduce(&mut state, dancers::actions::DancersAction::LoadFromGlobal);
            dancers::reducer::reduce(
                &mut state,
                dancers::actions::DancersAction::RequestSwapDancers,
            );

            check!(errors, !state.can_swap_dancers);
            check_eq!(errors, state.dancers.len(), 1);
            check_eq!(errors, state.dancers[0].name, "A");
            check!(errors, !state.is_dialog_open);

            assert_no_errors(errors);
        });
    });
    let report = dancers::run_suite(&suite);
    assert!(report.is_success());
}
