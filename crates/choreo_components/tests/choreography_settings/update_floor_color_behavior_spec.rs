use std::time::Duration;

use crate::choreography_settings;

use choreo_components::behavior::Behavior;
use choreo_components::choreography_settings::UpdateFloorColorBehavior;
use choreo_components::choreography_settings::UpdateFloorColorCommand;
use choreography_settings::Report;
use crossbeam_channel::unbounded;

#[test]
#[serial_test::serial]
fn update_floor_color_behavior_spec() {
    let suite = rspec::describe("update floor color behavior", (), |spec| {
        spec.it("updates floor color and sends redraw", |_| {
            let (redraw_sender, redraw_receiver) = unbounded();
            let context =
                choreography_settings::ChoreographySettingsTestContext::with_redraw_receiver(
                    redraw_receiver,
                );
            let (sender, receiver) = unbounded::<UpdateFloorColorCommand>();
            let behavior = UpdateFloorColorBehavior::new_with_receiver(
                context.global_state_store.clone(),
                redraw_sender,
                receiver,
            );
            context.activate_behaviors(vec![Box::new(behavior) as Box<dyn Behavior<_>>]);

            let color = choreo_master_mobile_json::Color {
                a: 255,
                r: 10,
                g: 20,
                b: 30,
            };
            sender
                .send(UpdateFloorColorCommand {
                    value: color.clone(),
                })
                .expect("send should succeed");

            let updated = context.wait_until(Duration::from_secs(1), || {
                context.read_global_state(|state| state.choreography.settings.floor_color == color)
            });
            assert!(updated);
            assert!(context.redraw_receiver.try_recv().is_ok());
        });
    });

    let report = choreography_settings::run_suite(&suite);
    assert!(report.is_success());
}
