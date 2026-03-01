use crate::dancers;
use dancers::Report;

#[test]
fn swap_dancers_behavior_spec() {
    let suite = rspec::describe("swap dancers behavior", (), |spec| {
        spec.it(
            "swaps role name shortcut color and icon while keeping dancer identities",
            |_| {
                let gentleman = dancers::role("Gentleman");
                let lady = dancers::role("Lady");
                let mut first =
                    dancers::dancer(1, gentleman.clone(), "Alex", "AL", Some("IconA"));
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
                dancers::reducer::reduce(&mut state, dancers::actions::DancersAction::SwapDancers);

                assert_eq!(state.dancers[0].dancer_id, 1);
                assert_eq!(state.dancers[1].dancer_id, 2);
                assert_eq!(state.dancers[0].role.name, "Lady");
                assert_eq!(state.dancers[1].role.name, "Gentleman");
                assert_eq!(state.dancers[0].name, "Bella");
                assert_eq!(state.dancers[1].name, "Alex");
                assert_eq!(state.dancers[0].shortcut, "BE");
                assert_eq!(state.dancers[1].shortcut, "AL");
                assert_eq!(state.dancers[0].color.r, 40);
                assert_eq!(state.dancers[1].color.r, 10);
                assert_eq!(state.dancers[0].icon.as_deref(), Some("IconB"));
                assert_eq!(state.dancers[1].icon.as_deref(), Some("IconA"));
            },
        );

        spec.it("does not swap when selection is invalid", |_| {
            let role = dancers::role("Role");
            let mut state = dancers::state::DancersState::default().with_global(
                dancers::state::DancersGlobalState {
                    dancers: vec![dancers::dancer(1, role, "A", "A", None)],
                    ..dancers::state::DancersGlobalState::default()
                },
            );

            dancers::reducer::reduce(
                &mut state,
                dancers::actions::DancersAction::LoadFromGlobal,
            );
            dancers::reducer::reduce(&mut state, dancers::actions::DancersAction::SwapDancers);

            assert!(!state.can_swap_dancers);
            assert_eq!(state.dancers.len(), 1);
            assert_eq!(state.dancers[0].name, "A");
        });
    });
    let report = dancers::run_suite(&suite);
    assert!(report.is_success());
}
