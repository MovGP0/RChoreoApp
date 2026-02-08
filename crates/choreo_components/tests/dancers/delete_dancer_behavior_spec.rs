use std::time::Duration;

use crate::dancers;

use dancers::Report;

#[test]
#[serial_test::serial]
fn delete_dancer_behavior_spec() {
    let suite = rspec::describe("delete dancer behavior", (), |spec| {
        spec.it("deletes selected dancer and keeps selection valid", |_| {
            let role = dancers::build_role("Role");
            let first = dancers::build_dancer(1, role.clone(), "A", "A", None);
            let second = dancers::build_dancer(2, role, "B", "B", None);
            let context = dancers::DancersTestContext::with_global_state(|state| {
                state.choreography.dancers = vec![first, second];
            });

            context.view_model.borrow_mut().delete_dancer();

            let deleted = context.wait_until(Duration::from_secs(1), || {
                context.view_model.borrow().dancers.len() == 1
            });
            assert!(deleted);

            {
                let view_model = context.view_model.borrow();
                assert_eq!(view_model.dancers[0].dancer_id.0, 2);
                assert_eq!(
                    view_model
                        .selected_dancer
                        .as_ref()
                        .map(|dancer| dancer.dancer_id.0),
                    Some(2)
                );
            }

            context.view_model.borrow_mut().delete_dancer();
            let emptied = context.wait_until(Duration::from_secs(1), || {
                context.view_model.borrow().dancers.is_empty()
            });
            assert!(emptied);
            assert!(context.view_model.borrow().selected_dancer.is_none());
        });
    });

    let report = dancers::run_suite(&suite);
    assert!(report.is_success());
}
