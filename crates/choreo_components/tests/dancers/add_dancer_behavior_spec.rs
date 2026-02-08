use std::time::Duration;

use crate::dancers;

use dancers::Report;

#[test]
#[serial_test::serial]
fn add_dancer_behavior_spec() {
    let suite = rspec::describe("add dancer behavior", (), |spec| {
        spec.it("adds a dancer with the next id and selects it", |_| {
            let role = dancers::build_role("Lead");
            let first = dancers::build_dancer(1, role.clone(), "A", "A", None);
            let context = dancers::DancersTestContext::with_global_state(|state| {
                state.choreography.roles = vec![role.clone()];
                state.choreography.dancers = vec![first];
            });

            context.view_model.borrow_mut().add_dancer();

            let added = context.wait_until(Duration::from_secs(1), || {
                context.view_model.borrow().dancers.len() == 2
            });
            assert!(added);

            let view_model = context.view_model.borrow();
            let selected = view_model
                .selected_dancer
                .as_ref()
                .expect("new dancer should be selected");
            assert_eq!(selected.dancer_id.0, 2);
            assert!(selected.name.is_empty());
            assert_eq!(selected.role.name, "Lead");
        });
    });

    let report = dancers::run_suite(&suite);
    assert!(report.is_success());
}
