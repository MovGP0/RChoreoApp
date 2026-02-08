use std::time::Duration;

use crate::dancers;

use dancers::Report;

#[test]
#[serial_test::serial]
fn swap_dancers_behavior_spec() {
    let suite = rspec::describe("swap dancers behavior", (), |spec| {
        spec.it("opens swap dialog when two different dancers are selected", |_| {
            let role = dancers::build_role("Role");
            let first = dancers::build_dancer(1, role.clone(), "A", "A", None);
            let second = dancers::build_dancer(2, role, "B", "B", None);
            let context = dancers::DancersTestContext::with_global_state(|state| {
                state.choreography.dancers = vec![first, second];
            });

            let ready = context.wait_until(Duration::from_secs(1), || {
                context.view_model.borrow().can_swap_dancers
            });
            assert!(ready);

            context.view_model.borrow_mut().swap_dancers();

            let shown = context.wait_until(Duration::from_secs(1), || {
                let view_model = context.view_model.borrow();
                view_model.is_dialog_open && view_model.dialog_content.as_deref() == Some("swap_dancers")
            });
            assert!(shown);
        });

        spec.it("does not open dialog when swap is not possible", |_| {
            let role = dancers::build_role("Role");
            let only = dancers::build_dancer(1, role, "A", "A", None);
            let context = dancers::DancersTestContext::with_global_state(|state| {
                state.choreography.dancers = vec![only];
            });

            context.view_model.borrow_mut().swap_dancers();
            context.pump_events();

            let view_model = context.view_model.borrow();
            assert!(!view_model.can_swap_dancers);
            assert!(!view_model.is_dialog_open);
            assert!(view_model.dialog_content.is_none());
        });
    });

    let report = dancers::run_suite(&suite);
    assert!(report.is_success());
}
