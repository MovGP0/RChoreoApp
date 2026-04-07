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
fn swap_dancer_selection_behavior_spec() {
    let suite = rspec::describe("swap dancer selection behavior", (), |spec| {
        spec.it("keeps swap candidates valid after list changes", |_| {
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
            let can_swap_after_load = state.can_swap_dancers;
            let swap_from_after_load = state.swap_from_dancer.is_some();
            let swap_to_after_load = state.swap_to_dancer.is_some();

            dancers::reducer::reduce(
                &mut state,
                dancers::actions::DancersAction::DeleteSelectedDancer,
            );

            let remaining_dancers = state.dancers.len();
            let can_swap_after_delete = state.can_swap_dancers;
            let swap_to_after_delete = state.swap_to_dancer.is_none();
            let swap_from_after_delete = state.swap_from_dancer.is_some();

            let mut errors = Vec::new();

            check!(errors, can_swap_after_load);
            check!(errors, swap_from_after_load);
            check!(errors, swap_to_after_load);
            check_eq!(errors, remaining_dancers, 1);
            check!(errors, !can_swap_after_delete);
            check!(errors, swap_to_after_delete);
            check!(errors, swap_from_after_delete);

            assert_no_errors(errors);
        });

        spec.it("disables swap when from and to are identical", |_| {
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
            let can_swap_after_load = state.can_swap_dancers;

            dancers::reducer::reduce(
                &mut state,
                dancers::actions::DancersAction::UpdateSwapFrom { index: 0 },
            );
            dancers::reducer::reduce(
                &mut state,
                dancers::actions::DancersAction::UpdateSwapTo { index: 0 },
            );
            let can_swap_when_identical = state.can_swap_dancers;

            dancers::reducer::reduce(
                &mut state,
                dancers::actions::DancersAction::UpdateSwapTo { index: 1 },
            );
            let can_swap_after_adjusting = state.can_swap_dancers;

            let mut errors = Vec::new();

            check!(errors, can_swap_after_load);
            check!(errors, !can_swap_when_identical);
            check!(errors, can_swap_after_adjusting);

            assert_no_errors(errors);
        });
    });
    let report = dancers::run_suite(&suite);
    assert!(report.is_success());
}
