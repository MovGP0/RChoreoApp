use std::time::Duration;

use crate::dancers;

use choreo_components::dancers::ShowDancerDialogCommand;
use dancers::Report;

#[test]
#[serial_test::serial]
fn hide_dancer_dialog_behavior_spec() {
    let suite = rspec::describe("hide dancer dialog behavior", (), |spec| {
        spec.it("clears dialog state when close command is received", |_| {
            let context = dancers::DancersTestContext::new();

            context
                .show_dialog_sender()
                .send(ShowDancerDialogCommand {
                    content_id: Some("swap_dancers".to_string()),
                })
                .expect("show dialog send should succeed");
            let opened = context.wait_until(Duration::from_secs(1), || {
                context.view_model.borrow().is_dialog_open
            });
            assert!(opened);

            context
                .close_dialog_sender()
                .send(choreo_components::dancers::CloseDancerDialogCommand)
                .expect("close dialog send should succeed");

            let closed = context.wait_until(Duration::from_secs(1), || {
                let view_model = context.view_model.borrow();
                !view_model.is_dialog_open && view_model.dialog_content.is_none()
            });
            assert!(closed);
        });
    });

    let report = dancers::run_suite(&suite);
    assert!(report.is_success());
}
