use std::time::Duration;

use crate::dancers;

use dancers::Report;

#[test]
#[serial_test::serial]
fn reload_dancer_settings_behavior_spec() {
    let suite = rspec::describe("reload dancer settings behavior", (), |spec| {
        spec.it(
            "reloads dancers from choreography and discards unsaved local edits",
            |_| {
                let role = dancers::build_role("Lead");
                let dancer = dancers::build_dancer(1, role.clone(), "Alice", "A", None);
                let context = dancers::DancersTestContext::with_global_state(|state| {
                    state.choreography.roles = vec![role];
                    state.choreography.dancers = vec![dancer];
                });

                let loaded = context.wait_until(Duration::from_secs(1), || {
                    context.view_model.borrow().dancers.len() == 1
                });
                assert!(loaded);

                context
                    .view_model
                    .borrow_mut()
                    .update_dancer_name("Edited Locally".to_string());
                context.pump_events();

                let original_name = context.read_global_state(|state| {
                    state
                        .choreography
                        .dancers
                        .first()
                        .map(|value| value.name.clone())
                        .unwrap_or_default()
                });
                assert_eq!(original_name, "Alice");

                context.reload_dancer_settings();
                let reloaded = context.wait_until(Duration::from_secs(1), || {
                    context
                        .view_model
                        .borrow()
                        .dancers
                        .first()
                        .map(|value| value.name.as_str() == "Alice")
                        .unwrap_or(false)
                });
                assert!(reloaded);
            },
        );
    });

    let report = dancers::run_suite(&suite);
    assert!(report.is_success());
}
