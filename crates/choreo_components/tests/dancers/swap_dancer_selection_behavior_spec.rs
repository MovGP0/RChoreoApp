use std::time::Duration;

use crate::dancers;

use dancers::Report;

#[test]
#[serial_test::serial]
fn swap_dancer_selection_behavior_spec() {
    let suite = rspec::describe("swap dancer selection behavior", (), |spec| {
        spec.it(
            "keeps swap candidates valid after dancer list changes",
            |_| {
                let role = dancers::build_role("Role");
                let first = dancers::build_dancer(1, role.clone(), "A", "A", None);
                let second = dancers::build_dancer(2, role, "B", "B", None);
                let context = dancers::DancersTestContext::with_global_state(|state| {
                    state.choreography.dancers = vec![first, second];
                });

                let initialized = context.wait_until(Duration::from_secs(1), || {
                    let view_model = context.view_model.borrow();
                    view_model.can_swap_dancers
                        && view_model.swap_from_dancer.is_some()
                        && view_model.swap_to_dancer.is_some()
                });
                assert!(initialized);

                context.view_model.borrow_mut().delete_dancer();

                let updated = context.wait_until(Duration::from_secs(1), || {
                    let view_model = context.view_model.borrow();
                    view_model.dancers.len() == 1
                        && !view_model.can_swap_dancers
                        && view_model.swap_to_dancer.is_none()
                        && view_model.swap_from_dancer.is_some()
                });
                assert!(updated);
            },
        );

        spec.it(
            "disables swap when the same dancer is selected for both selectors",
            |_| {
                let role = dancers::build_role("Role");
                let first = dancers::build_dancer(1, role.clone(), "A", "A", None);
                let second = dancers::build_dancer(2, role, "B", "B", None);
                let context = dancers::DancersTestContext::with_global_state(|state| {
                    state.choreography.dancers = vec![first, second];
                });

                let initialized = context.wait_until(Duration::from_secs(1), || {
                    context.view_model.borrow().can_swap_dancers
                });
                assert!(initialized);

                context.view_model.borrow_mut().update_swap_from(0);
                context.view_model.borrow_mut().update_swap_to(0);

                let disabled = context.wait_until(Duration::from_secs(1), || {
                    let view_model = context.view_model.borrow();
                    !view_model.can_swap_dancers
                        && view_model
                            .swap_from_dancer
                            .as_ref()
                            .zip(view_model.swap_to_dancer.as_ref())
                            .map(|(from, to)| from.dancer_id == to.dancer_id)
                            .unwrap_or(false)
                });
                assert!(disabled);

                context.view_model.borrow_mut().update_swap_to(1);
                let enabled = context.wait_until(Duration::from_secs(1), || {
                    context.view_model.borrow().can_swap_dancers
                });
                assert!(enabled);
            },
        );
    });

    let report = dancers::run_suite(&suite);
    assert!(report.is_success());
}
