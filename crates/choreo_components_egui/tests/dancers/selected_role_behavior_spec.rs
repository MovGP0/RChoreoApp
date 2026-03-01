use crate::dancers;
use dancers::Report;

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

            assert_eq!(
                state
                    .selected_dancer
                    .as_ref()
                    .map(|dancer| dancer.role.name.as_str()),
                Some("Follow")
            );
            assert_eq!(state.dancers[0].role.name, "Follow");
            assert_eq!(
                state.selected_role.as_ref().map(|role| role.name.as_str()),
                Some("Follow")
            );
        });
    });
    let report = dancers::run_suite(&suite);
    assert!(report.is_success());
}
