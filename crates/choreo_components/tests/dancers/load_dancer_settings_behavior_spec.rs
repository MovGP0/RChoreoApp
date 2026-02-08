use std::rc::Rc;
use std::time::Duration;

use crate::dancers;

use dancers::Report;

#[test]
#[serial_test::serial]
fn load_dancer_settings_behavior_spec() {
    let suite = rspec::describe("load dancer settings behavior", (), |spec| {
        spec.it("loads roles and dancers from global state on activation", |_| {
            let role_a = dancers::build_role("Lead");
            let role_b = dancers::build_role("Follow");
            let dancer_a = dancers::build_dancer(1, role_a.clone(), "Alice", "A", None);
            let dancer_b = dancers::build_dancer(2, role_b.clone(), "Bob", "B", None);

            let context = dancers::DancersTestContext::with_global_state(|state| {
                state.choreography.roles = vec![role_a.clone(), role_b.clone()];
                state.choreography.dancers = vec![dancer_a.clone(), dancer_b];
            });

            let loaded = context.wait_until(Duration::from_secs(1), || {
                let view_model = context.view_model.borrow();
                view_model.roles.len() >= 2 && view_model.dancers.len() == 2
            });
            assert!(loaded);

            let view_model = context.view_model.borrow();
            assert_eq!(view_model.roles[0].name, "Lead");
            assert_eq!(view_model.roles[1].name, "Follow");
            assert!(!Rc::ptr_eq(&view_model.roles[0], &role_a));

            assert_eq!(view_model.dancers[0].name, "Alice");
            assert_eq!(
                view_model
                    .selected_dancer
                    .as_ref()
                    .map(|dancer| dancer.dancer_id.0),
                Some(dancer_a.dancer_id.0)
            );
        });
    });

    let report = dancers::run_suite(&suite);
    assert!(report.is_success());
}
