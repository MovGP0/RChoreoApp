use std::time::Duration;

use crate::dancers;

use dancers::Report;

#[test]
#[serial_test::serial]
fn selected_icon_behavior_spec() {
    let suite = rspec::describe("selected icon behavior", (), |spec| {
        spec.it("updates selected dancer icon and list item icon", |_| {
            let role = dancers::build_role("Role");
            let dancer = dancers::build_dancer(1, role, "A", "A", None);
            let context = dancers::DancersTestContext::with_global_state(|state| {
                state.choreography.dancers = vec![dancer];
            });

            let key = context
                .view_model
                .borrow()
                .icon_options
                .first()
                .map(|option| option.key.clone())
                .expect("icon options should not be empty");
            context
                .view_model
                .borrow_mut()
                .update_dancer_icon(key.clone());

            let updated = context.wait_until(Duration::from_secs(1), || {
                context
                    .view_model
                    .borrow()
                    .selected_dancer
                    .as_ref()
                    .and_then(|dancer| dancer.icon.as_ref().cloned())
                    == Some(key.clone())
            });
            assert!(updated);

            let view_model = context.view_model.borrow();
            assert_eq!(view_model.dancers[0].icon.as_deref(), Some(key.as_str()));
        });
    });

    let report = dancers::run_suite(&suite);
    assert!(report.is_success());
}
