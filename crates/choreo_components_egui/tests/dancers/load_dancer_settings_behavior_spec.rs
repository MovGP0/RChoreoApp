use crate::dancers;
use dancers::Report;

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

            assert_eq!(state.roles.len(), 2);
            assert_eq!(state.roles[0].name, "Lead");
            assert_eq!(state.dancers.len(), 2);
            assert_eq!(state.dancers[0].name, "Alice");
            assert_eq!(
                state.selected_dancer.as_ref().map(|dancer| dancer.dancer_id),
                Some(1)
            );
        });
    });
    let report = dancers::run_suite(&suite);
    assert!(report.is_success());
}
