use crate::dancers;
use dancers::Report;

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
            assert!(state.can_swap_dancers);
            assert!(state.swap_from_dancer.is_some());
            assert!(state.swap_to_dancer.is_some());

            dancers::reducer::reduce(
                &mut state,
                dancers::actions::DancersAction::DeleteSelectedDancer,
            );

            assert_eq!(state.dancers.len(), 1);
            assert!(!state.can_swap_dancers);
            assert!(state.swap_to_dancer.is_none());
            assert!(state.swap_from_dancer.is_some());
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
            assert!(state.can_swap_dancers);

            dancers::reducer::reduce(
                &mut state,
                dancers::actions::DancersAction::UpdateSwapFrom { index: 0 },
            );
            dancers::reducer::reduce(
                &mut state,
                dancers::actions::DancersAction::UpdateSwapTo { index: 0 },
            );
            assert!(!state.can_swap_dancers);

            dancers::reducer::reduce(
                &mut state,
                dancers::actions::DancersAction::UpdateSwapTo { index: 1 },
            );
            assert!(state.can_swap_dancers);
        });
    });
    let report = dancers::run_suite(&suite);
    assert!(report.is_success());
}
