use crate::dancers;
use dancers::Report;

#[test]
fn selected_icon_behavior_spec() {
    let suite = rspec::describe("selected icon behavior", (), |spec| {
        spec.it("updates selected dancer and list icon values", |_| {
            let role = dancers::role("Role");
            let mut state = dancers::state::DancersState::default().with_global(
                dancers::state::DancersGlobalState {
                    dancers: vec![dancers::dancer(1, role, "A", "A", None)],
                    ..dancers::state::DancersGlobalState::default()
                },
            );

            dancers::reducer::reduce(&mut state, dancers::actions::DancersAction::LoadFromGlobal);
            let key = state
                .icon_options
                .first()
                .map(|option| option.key.clone())
                .expect("icon options should not be empty");

            dancers::reducer::reduce(
                &mut state,
                dancers::actions::DancersAction::UpdateDancerIcon { value: key.clone() },
            );

            assert_eq!(
                state
                    .selected_dancer
                    .as_ref()
                    .and_then(|dancer| dancer.icon.as_deref()),
                Some(key.as_str())
            );
            assert_eq!(state.dancers[0].icon.as_deref(), Some(key.as_str()));
        });
    });
    let report = dancers::run_suite(&suite);
    assert!(report.is_success());
}
