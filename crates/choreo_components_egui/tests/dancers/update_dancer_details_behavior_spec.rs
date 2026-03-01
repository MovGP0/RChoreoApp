use crate::dancers;
use dancers::Report;

#[test]
fn update_dancer_details_behavior_spec() {
    let suite = rspec::describe("update dancer details behavior", (), |spec| {
        spec.it("updates selected dancer and list values", |_| {
            let role = dancers::role("Gentleman");
            let mut state = dancers::state::DancersState::default().with_global(
                dancers::state::DancersGlobalState {
                    dancers: vec![dancers::dancer(1, role, "Alice", "A", None)],
                    ..dancers::state::DancersGlobalState::default()
                },
            );

            dancers::reducer::reduce(&mut state, dancers::actions::DancersAction::LoadFromGlobal);
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

            let selected = state
                .selected_dancer
                .as_ref()
                .expect("selected dancer should exist");
            assert_eq!(selected.name, "Alice Updated");
            assert_eq!(selected.shortcut, "AU");
            assert_eq!(selected.color.r, 10);
            assert_eq!(state.dancers[0].name, "Alice Updated");
            assert_eq!(state.dancers[0].shortcut, "AU");
            assert_eq!(state.dancers[0].color.r, 10);
        });
    });
    let report = dancers::run_suite(&suite);
    assert!(report.is_success());
}
