use std::time::Duration;

use crate::choreography_settings;

use choreo_components::behavior::Behavior;
use choreo_components::choreography_settings::UpdateDateBehavior;
use choreo_components::choreography_settings::UpdateDateCommand;
use choreography_settings::Report;
use crossbeam_channel::unbounded;
use time::Date;
use time::Month;

#[test]
#[serial_test::serial]
fn update_date_behavior_spec() {
    let suite = rspec::describe("update date behavior", (), |spec| {
        spec.it("updates choreography date and sends redraw", |_| {
            let (redraw_sender, redraw_receiver) = unbounded();
            let context =
                choreography_settings::ChoreographySettingsTestContext::with_redraw_receiver(
                    redraw_receiver,
                );
            let (sender, receiver) = unbounded::<UpdateDateCommand>();
            let behavior = UpdateDateBehavior::new_with_receiver(
                context.global_state_store.clone(),
                redraw_sender,
                receiver,
            );
            context.activate_behaviors(vec![Box::new(behavior) as Box<dyn Behavior<_>>]);

            let date = Date::from_calendar_date(2026, Month::January, 25).expect("valid date");
            sender
                .send(UpdateDateCommand { value: date })
                .expect("send should succeed");

            let updated = context.wait_until(Duration::from_secs(1), || {
                context.read_global_state(|state| state.choreography.date) == Some(date)
            });
            assert!(updated);
            assert!(context.redraw_receiver.try_recv().is_ok());
        });
    });

    let report = choreography_settings::run_suite(&suite);
    assert!(report.is_success());
}
