use crate::dancers;
use dancers::Report;

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
            assert_eq!(state.dancers[0].name, "Edited Locally");

            dancers::reducer::reduce(
                &mut state,
                dancers::actions::DancersAction::ReloadFromGlobal,
            );
            assert_eq!(state.dancers[0].name, "Alice");
        });
    });
    let report = dancers::run_suite(&suite);
    assert!(report.is_success());
}
