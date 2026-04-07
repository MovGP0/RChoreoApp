use crate::dancers;
use dancers::Report;

#[test]
fn update_dancer_details_behavior_spec() {
    let suite = rspec::describe("update dancer details behavior", (), |spec| {
        spec.it(
            "updates selected dancer name shortcut and color and propagates to list",
            |_| {
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
                            $errors.push(format!(
                                "condition failed: {}",
                                stringify!($condition)
                            ));
                        }
                    };
                }

                let role = dancers::role("Gentleman");
                let mut state = dancers::state::DancersState::default().with_global(
                    dancers::state::DancersGlobalState {
                        dancers: vec![dancers::dancer(1, role, "Alice", "A", None)],
                        ..dancers::state::DancersGlobalState::default()
                    },
                );

                dancers::reducer::reduce(
                    &mut state,
                    dancers::actions::DancersAction::LoadFromGlobal,
                );
                dancers::reducer::reduce(
                    &mut state,
                    dancers::actions::DancersAction::UpdateDancerName {
                        value: "Alice Updated".to_string(),
                    },
                );
                dancers::reducer::reduce(
                    &mut state,
                    dancers::actions::DancersAction::UpdateDancerShortcut {
                        value: "AU".to_string(),
                    },
                );
                dancers::reducer::reduce(
                    &mut state,
                    dancers::actions::DancersAction::UpdateDancerColor {
                        value: dancers::color(10, 20, 30),
                    },
                );

                let mut errors = Vec::new();

                check!(errors, state.selected_dancer.is_some());

                if let Some(selected) = state.selected_dancer.as_ref() {
                    check_eq!(errors, selected.name, "Alice Updated");
                    check_eq!(errors, selected.shortcut, "AU");
                    check_eq!(errors, selected.color.r, 10);
                    check_eq!(errors, selected.color.g, 20);
                    check_eq!(errors, selected.color.b, 30);
                }

                check_eq!(errors, state.dancers[0].name, "Alice Updated");
                check_eq!(errors, state.dancers[0].shortcut, "AU");
                check_eq!(errors, state.dancers[0].color.r, 10);
                check_eq!(errors, state.dancers[0].color.g, 20);
                check_eq!(errors, state.dancers[0].color.b, 30);

                assert!(
                    errors.is_empty(),
                    "Assertion failures:\n{}",
                    errors.join("\n")
                );
            },
        );
    });
    let report = dancers::run_suite(&suite);
    assert!(report.is_success());
}
