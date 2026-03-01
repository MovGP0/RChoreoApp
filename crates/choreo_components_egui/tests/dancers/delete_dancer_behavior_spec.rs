use crate::dancers;
use dancers::Report;

#[test]
fn delete_dancer_behavior_spec() {
    let suite = rspec::describe("delete dancer behavior", (), |spec| {
        spec.it("deletes selected dancer and keeps selection valid", |_| {
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
            dancers::reducer::reduce(
                &mut state,
                dancers::actions::DancersAction::DeleteSelectedDancer,
            );

            assert_eq!(state.dancers.len(), 1);
            assert_eq!(state.dancers[0].dancer_id, 2);
            assert_eq!(
                state.selected_dancer.as_ref().map(|dancer| dancer.dancer_id),
                Some(2)
            );

            dancers::reducer::reduce(
                &mut state,
                dancers::actions::DancersAction::DeleteSelectedDancer,
            );
            assert!(state.dancers.is_empty());
            assert!(state.selected_dancer.is_none());
        });
    });
    let report = dancers::run_suite(&suite);
    assert!(report.is_success());
}
