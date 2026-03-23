use crate::dancers;
use dancers::Report;

#[test]
fn add_dancer_behavior_spec() {
    let suite = rspec::describe("add dancer behavior", (), |spec| {
        spec.it("adds the next dancer and selects it", |_| {
            let lead = dancers::role("Lead");
            let mut state = dancers::state::DancersState::default().with_global(
                dancers::state::DancersGlobalState {
                    roles: vec![lead.clone()],
                    dancers: vec![dancers::dancer(1, lead, "A", "A", None)],
                    ..dancers::state::DancersGlobalState::default()
                },
            );

            dancers::reducer::reduce(&mut state, dancers::actions::DancersAction::LoadFromGlobal);
            dancers::reducer::reduce(&mut state, dancers::actions::DancersAction::AddDancer);

            assert_eq!(state.dancers.len(), 2);
            let selected = state
                .selected_dancer
                .as_ref()
                .expect("new dancer should be selected");
            assert_eq!(selected.dancer_id, 2);
            assert!(selected.name.is_empty());
            assert_eq!(selected.role.name, "Lead");
        });
    });
    let report = dancers::run_suite(&suite);
    assert!(report.is_success());
}
