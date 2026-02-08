use std::time::Duration;

use crate::choreography_settings;

use choreo_components::behavior::Behavior;
use choreo_components::choreography_settings::UpdateFloorLeftBehavior;
use choreo_components::choreography_settings::UpdateFloorLeftCommand;
use choreography_settings::Report;
use crossbeam_channel::unbounded;

#[test]
#[serial_test::serial]
fn update_floor_left_behavior_spec() {
    let suite = rspec::describe("update floor left behavior", (), |spec| {
        spec.it("clamps floor left range and sends redraw", |_| {
            let (redraw_sender, redraw_receiver) = unbounded();
            let context =
                choreography_settings::ChoreographySettingsTestContext::with_redraw_receiver(
                    redraw_receiver,
                );
            let (sender, receiver) = unbounded::<UpdateFloorLeftCommand>();
            let behavior = UpdateFloorLeftBehavior::new_with_receiver(
                context.global_state_store.clone(),
                redraw_sender,
                receiver,
            );
            context.activate_behaviors(vec![Box::new(behavior) as Box<dyn Behavior<_>>]);

            sender
                .send(UpdateFloorLeftCommand { value: 101 })
                .expect("send should succeed");

            let updated = context.wait_until(Duration::from_secs(1), || {
                context.read_global_state(|state| state.choreography.floor.size_left == 100)
            });
            assert!(updated);
            assert!(context.redraw_receiver.try_recv().is_ok());
        });
    });

    let report = choreography_settings::run_suite(&suite);
    assert!(report.is_success());
}
