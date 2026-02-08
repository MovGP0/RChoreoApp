use std::time::Duration;

use crate::choreography_settings;

use choreo_components::behavior::Behavior;
use choreo_components::choreography_settings::ShowTimestampsChangedEvent;
use choreo_components::choreography_settings::UpdateShowTimestampsBehavior;
use choreo_components::choreography_settings::UpdateShowTimestampsCommand;
use choreo_components::preferences::Preferences;
use choreo_models::SettingsPreferenceKeys;
use crossbeam_channel::unbounded;
use choreography_settings::Report;

#[test]
#[serial_test::serial]
fn update_show_timestamps_behavior_spec() {
    let suite = rspec::describe("update show timestamps behavior", (), |spec| {
        spec.it("initializes from preferences and broadcasts changes with redraw", |_| {
            let preferences = choreo_components::preferences::InMemoryPreferences::new();
            preferences.set_bool(SettingsPreferenceKeys::SHOW_TIMESTAMPS, false);

            let (redraw_sender, redraw_receiver) = unbounded();
            let (show_sender, show_receiver) = unbounded::<ShowTimestampsChangedEvent>();
            let context = choreography_settings::ChoreographySettingsTestContext::with_redraw_receiver(redraw_receiver);
            let (sender, receiver) = unbounded::<UpdateShowTimestampsCommand>();
            let behavior = UpdateShowTimestampsBehavior::new_with_receiver(
                context.global_state_store.clone(),
                preferences,
                redraw_sender,
                show_sender,
                receiver,
            );
            context.activate_behaviors(vec![Box::new(behavior) as Box<dyn Behavior<_>>]);

            assert!(!context.view_model.borrow().show_timestamps);

            sender
                .send(UpdateShowTimestampsCommand { value: true })
                .expect("send should succeed");

            let updated = context.wait_until(Duration::from_secs(1), || {
                context.view_model.borrow().show_timestamps
                    && context.read_global_state(|state| state.choreography.settings.show_timestamps)
            });
            assert!(updated);
            assert!(context.redraw_receiver.try_recv().is_ok());

            let event = show_receiver.try_recv().expect("show-timestamps event should be emitted");
            assert!(event.is_enabled);
        });
    });

    let report = choreography_settings::run_suite(&suite);
    assert!(report.is_success());
}
