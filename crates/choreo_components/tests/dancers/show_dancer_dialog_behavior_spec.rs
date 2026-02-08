use std::time::Duration;

use crate::dancers;

use choreo_components::dancers::ShowDancerDialogCommand;
use dancers::Report;

#[test]
#[serial_test::serial]
fn show_dancer_dialog_behavior_spec() {
    let suite = rspec::describe("show dancer dialog behavior", (), |spec| {
        spec.it("applies dialog visibility from command payload", |_| {
            let context = dancers::DancersTestContext::new();

            context
                .show_dialog_sender()
                .send(ShowDancerDialogCommand {
                    content_id: Some("swap_dancers".to_string()),
                })
                .expect("show dialog send should succeed");

            let opened = context.wait_until(Duration::from_secs(1), || {
                let view_model = context.view_model.borrow();
                view_model.is_dialog_open
                    && view_model.dialog_content.as_deref() == Some("swap_dancers")
            });
            assert!(opened);

            context
                .show_dialog_sender()
                .send(ShowDancerDialogCommand { content_id: None })
                .expect("show dialog send should succeed");

            let cleared = context.wait_until(Duration::from_secs(1), || {
                let view_model = context.view_model.borrow();
                !view_model.is_dialog_open && view_model.dialog_content.is_none()
            });
            assert!(cleared);
        });
    });

    let report = dancers::run_suite(&suite);
    assert!(report.is_success());
}
