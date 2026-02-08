use std::time::Duration;

use crate::choreo_main;

use choreo_components::behavior::Behavior;
use choreo_components::choreo_main::ShowDialogBehavior;
use choreo_components::choreo_main::ShowDialogCommand;
use choreo_main::Report;
use crossbeam_channel::unbounded;

#[test]
#[serial_test::serial]
fn show_dialog_behavior_spec() {
    let suite = rspec::describe("show dialog behavior", (), |spec| {
        spec.it("opens dialog with provided content", |_| {
            let (sender, receiver) = unbounded::<ShowDialogCommand>();
            let behavior = ShowDialogBehavior::new(receiver);
            let context = choreo_main::ChoreoMainTestContext::new(vec![
                Box::new(behavior) as Box<dyn Behavior<_>>
            ]);

            sender
                .send(ShowDialogCommand {
                    content: Some("dialog content".to_string()),
                })
                .expect("send should succeed");

            let shown = context.wait_until(Duration::from_secs(1), || {
                let vm = context.view_model.borrow();
                vm.is_dialog_open && vm.dialog_content.as_deref() == Some("dialog content")
            });
            assert!(shown);
        });
    });

    let report = choreo_main::run_suite(&suite);
    assert!(report.is_success());
}
