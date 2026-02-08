use std::time::Duration;

use crate::choreography_settings;

use choreo_components::behavior::Behavior;
use choreo_components::choreography_settings::UpdateTransparencyBehavior;
use choreo_components::choreography_settings::UpdateTransparencyCommand;
use crossbeam_channel::unbounded;
use choreography_settings::Report;

#[test]
#[serial_test::serial]
fn update_transparency_behavior_spec() {
    let suite = rspec::describe("update transparency behavior", (), |spec| {
        spec.it("clamps transparency to 0..1 and sends redraw", |_| {
            let (redraw_sender, redraw_receiver) = unbounded();
            let context = choreography_settings::ChoreographySettingsTestContext::with_redraw_receiver(redraw_receiver);
            let (sender, receiver) = unbounded::<UpdateTransparencyCommand>();
            let behavior = UpdateTransparencyBehavior::new_with_receiver(
                context.global_state_store.clone(),
                redraw_sender,
                receiver,
            );
            context.activate_behaviors(vec![Box::new(behavior) as Box<dyn Behavior<_>>]);

            sender
                .send(UpdateTransparencyCommand { value: 2.0 })
                .expect("send should succeed");

            let updated = context.wait_until(Duration::from_secs(1), || {
                context.read_global_state(|state| (state.choreography.settings.transparency - 1.0).abs() < 0.0001)
            });
            assert!(updated);
            assert!(context.redraw_receiver.try_recv().is_ok());
        });
    });

    let report = choreography_settings::run_suite(&suite);
    assert!(report.is_success());
}
