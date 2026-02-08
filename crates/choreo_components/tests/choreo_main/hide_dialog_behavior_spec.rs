use std::time::Duration;

use crate::choreo_main;

use choreo_components::behavior::Behavior;
use choreo_components::choreo_main::HideDialogBehavior;
use choreo_components::choreo_main::CloseDialogCommand;
use crossbeam_channel::unbounded;
use choreo_main::Report;

#[test]
#[serial_test::serial]
fn hide_dialog_behavior_spec() {
    let suite = rspec::describe("hide dialog behavior", (), |spec| {
        spec.it("closes dialog when close command is received", |_| {
            let (sender, receiver) = unbounded::<CloseDialogCommand>();
            let behavior = HideDialogBehavior::new(receiver);
            let context = choreo_main::ChoreoMainTestContext::new(vec![
                Box::new(behavior) as Box<dyn Behavior<_>>,
            ]);

            {
                let mut vm = context.view_model.borrow_mut();
                vm.show_dialog(Some("hello".to_string()));
            }

            sender.send(CloseDialogCommand).expect("send should succeed");

            let closed = context.wait_until(Duration::from_secs(1), || {
                let vm = context.view_model.borrow();
                !vm.is_dialog_open && vm.dialog_content.is_none()
            });
            assert!(closed);
        });
    });

    let report = choreo_main::run_suite(&suite);
    assert!(report.is_success());
}
