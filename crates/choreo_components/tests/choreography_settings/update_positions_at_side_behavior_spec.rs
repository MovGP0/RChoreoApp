use std::time::Duration;

use crate::choreography_settings;

use choreo_components::behavior::Behavior;
use choreo_components::choreography_settings::UpdatePositionsAtSideBehavior;
use choreo_components::choreography_settings::UpdatePositionsAtSideCommand;
use choreo_components::preferences::Preferences;
use choreo_models::SettingsPreferenceKeys;
use choreography_settings::Report;
use crossbeam_channel::unbounded;

#[test]
#[serial_test::serial]
fn update_positions_at_side_behavior_spec() {
    let suite = rspec::describe("update positions at side behavior", (), |spec| {
        spec.it(
            "initializes from preferences and updates global state with redraw",
            |_| {
                let preferences = choreo_components::preferences::InMemoryPreferences::new();
                preferences.set_bool(SettingsPreferenceKeys::POSITIONS_AT_SIDE, false);

                let (redraw_sender, redraw_receiver) = unbounded();
                let context =
                    choreography_settings::ChoreographySettingsTestContext::with_redraw_receiver(
                        redraw_receiver,
                    );
                let (sender, receiver) = unbounded::<UpdatePositionsAtSideCommand>();
                let behavior = UpdatePositionsAtSideBehavior::new_with_receiver(
                    context.global_state_store.clone(),
                    preferences,
                    redraw_sender,
                    receiver,
                );
                context.activate_behaviors(vec![Box::new(behavior) as Box<dyn Behavior<_>>]);

                assert!(!context.view_model.borrow().positions_at_side);

                sender
                    .send(UpdatePositionsAtSideCommand { value: true })
                    .expect("send should succeed");

                let updated = context.wait_until(Duration::from_secs(1), || {
                    context.read_global_state(|state| state.choreography.settings.positions_at_side)
                        && context.view_model.borrow().positions_at_side
                });
                assert!(updated);
                assert!(context.redraw_receiver.try_recv().is_ok());
            },
        );
    });

    let report = choreography_settings::run_suite(&suite);
    assert!(report.is_success());
}
