use crate::dancers;
use crate::dancers::Report;

#[test]
fn dancer_settings_page_control_spec() {
    let suite = rspec::describe("dancer settings page control", (), |spec| {
        spec.it("keeps dancer list open by default", |_| {
            let state = dancers::state::DancersState::default();
            assert!(state.is_dancer_list_open);
        });

        spec.it("updates drawer visibility from reducer actions", |_| {
            let mut state = dancers::state::DancersState::default();

            dancers::reducer::reduce(
                &mut state,
                dancers::actions::DancersAction::ToggleDancerList,
            );
            assert!(!state.is_dancer_list_open);

            dancers::reducer::reduce(
                &mut state,
                dancers::actions::DancersAction::OpenDancerList,
            );
            assert!(state.is_dancer_list_open);

            dancers::reducer::reduce(
                &mut state,
                dancers::actions::DancersAction::CloseDancerList,
            );
            assert!(!state.is_dancer_list_open);
        });
    });

    let report = dancers::run_suite(&suite);
    assert!(report.is_success());
}
