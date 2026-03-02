use crate::dancers;
use dancers::Report;

#[test]
fn selected_dancer_state_behavior_spec() {
    let suite = rspec::describe("selected dancer state behavior", (), |spec| {
        spec.it(
            "updates selected state, role, and icon option after selection",
            |_| {
                let role = dancers::role("Role");
                let expected_icon_key = dancers::state::default_icon_options()
                    .first()
                    .expect("icon options exist")
                    .key
                    .clone();
                let mut state = dancers::state::DancersState::default().with_global(
                    dancers::state::DancersGlobalState {
                        roles: vec![role.clone()],
                        dancers: vec![
                            dancers::dancer(1, role.clone(), "A", "A", None),
                            dancers::dancer(2, role, "B", "B", Some(expected_icon_key.as_str())),
                        ],
                        ..dancers::state::DancersGlobalState::default()
                    },
                );

                dancers::reducer::reduce(
                    &mut state,
                    dancers::actions::DancersAction::LoadFromGlobal,
                );
                dancers::reducer::reduce(
                    &mut state,
                    dancers::actions::DancersAction::SelectDancer { index: 1 },
                );

                assert!(state.has_selected_dancer);
                assert!(state.can_delete_dancer);
                assert_eq!(
                    state
                        .selected_icon_option
                        .as_ref()
                        .map(|value| value.key.as_str()),
                    Some(expected_icon_key.as_str())
                );
                assert_eq!(
                    state
                        .selected_role
                        .as_ref()
                        .map(|value| value.name.as_str()),
                    Some("Role")
                );
            },
        );
    });
    let report = dancers::run_suite(&suite);
    assert!(report.is_success());
}
