use std::time::Duration;

use crate::dancers;

use dancers::Report;

#[test]
#[serial_test::serial]
fn selected_role_behavior_spec() {
    let suite = rspec::describe("selected role behavior", (), |spec| {
        spec.it("updates selected dancer role using selected index", |_| {
            let lead = dancers::build_role("Lead");
            let follow = dancers::build_role("Follow");
            let dancer = dancers::build_dancer(1, lead.clone(), "A", "A", None);
            let context = dancers::DancersTestContext::with_global_state(|state| {
                state.choreography.roles = vec![lead, follow];
                state.choreography.dancers = vec![dancer];
            });

            context.view_model.borrow_mut().select_role(1);

            let updated = context.wait_until(Duration::from_secs(1), || {
                context
                    .view_model
                    .borrow()
                    .selected_dancer
                    .as_ref()
                    .map(|dancer| dancer.role.name.as_str())
                    == Some("Follow")
            });
            assert!(updated);

            let view_model = context.view_model.borrow();
            assert_eq!(view_model.dancers[0].role.name, "Follow");
            assert_eq!(
                view_model
                    .selected_role
                    .as_ref()
                    .map(|role| role.name.as_str()),
                Some("Follow")
            );
        });
    });

    let report = dancers::run_suite(&suite);
    assert!(report.is_success());
}
