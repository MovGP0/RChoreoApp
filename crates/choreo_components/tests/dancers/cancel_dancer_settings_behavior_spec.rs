use crate::dancers;

use dancers::Report;

#[test]
#[serial_test::serial]
fn cancel_dancer_settings_behavior_spec() {
    let suite = rspec::describe("cancel dancer settings behavior", (), |spec| {
        spec.it("consumes cancel command without mutating dialog state", |_| {
            let context = dancers::DancersTestContext::new();
            {
                let mut view_model = context.view_model.borrow_mut();
                view_model.is_dialog_open = true;
                view_model.dialog_content = Some("swap_dancers".to_string());
            }

            context.view_model.borrow_mut().cancel();
            context.pump_events();

            let view_model = context.view_model.borrow();
            assert!(view_model.is_dialog_open);
            assert_eq!(view_model.dialog_content.as_deref(), Some("swap_dancers"));
        });
    });

    let report = dancers::run_suite(&suite);
    assert!(report.is_success());
}
