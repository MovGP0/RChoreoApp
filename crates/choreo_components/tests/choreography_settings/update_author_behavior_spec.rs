use std::time::Duration;

use crate::choreography_settings;

use choreo_components::behavior::Behavior;
use choreo_components::choreography_settings::UpdateAuthorBehavior;
use choreo_components::choreography_settings::UpdateAuthorCommand;
use crossbeam_channel::unbounded;
use choreography_settings::Report;

#[test]
#[serial_test::serial]
fn update_author_behavior_spec() {
    let suite = rspec::describe("update author behavior", (), |spec| {
        spec.it("updates choreography author and sends redraw", |_| {
            let (redraw_sender, redraw_receiver) = unbounded();
            let context = choreography_settings::ChoreographySettingsTestContext::with_redraw_receiver(redraw_receiver);
            let (sender, receiver) = unbounded::<UpdateAuthorCommand>();
            let behavior = UpdateAuthorBehavior::new_with_receiver(
                context.global_state_store.clone(),
                redraw_sender,
                receiver,
            );
            context.activate_behaviors(vec![Box::new(behavior) as Box<dyn Behavior<_>>]);

            sender
                .send(UpdateAuthorCommand {
                    value: "  Jane Doe  ".to_string(),
                })
                .expect("send should succeed");

            let updated = context.wait_until(Duration::from_secs(1), || {
                context.read_global_state(|state| state.choreography.author.as_deref() == Some("Jane Doe"))
            });
            assert!(updated);
            assert!(context.redraw_receiver.try_recv().is_ok());
        });
    });

    let report = choreography_settings::run_suite(&suite);
    assert!(report.is_success());
}
