use std::time::Duration;

use crate::choreography_settings;

use choreo_components::behavior::Behavior;
use choreo_components::choreography_settings::UpdateSnapToGridBehavior;
use choreo_components::choreography_settings::UpdateSnapToGridCommand;
use choreo_components::preferences::Preferences;
use choreo_models::SettingsPreferenceKeys;
use choreography_settings::Report;
use crossbeam_channel::unbounded;

#[test]
#[serial_test::serial]
fn update_snap_to_grid_behavior_spec() {
    let suite = rspec::describe("update snap to grid behavior", (), |spec| {
        spec.it(
            "initializes from preferences and updates value with redraw",
            |_| {
                let preferences = choreo_components::preferences::InMemoryPreferences::new();
                preferences.set_bool(SettingsPreferenceKeys::SNAP_TO_GRID, false);

                let (redraw_sender, redraw_receiver) = unbounded();
                let context =
                    choreography_settings::ChoreographySettingsTestContext::with_redraw_receiver(
                        redraw_receiver,
                    );
                let (sender, receiver) = unbounded::<UpdateSnapToGridCommand>();
                let behavior = UpdateSnapToGridBehavior::new_with_receiver(
                    context.global_state_store.clone(),
                    preferences.clone(),
                    redraw_sender,
                    receiver,
                );
                context.activate_behaviors(vec![Box::new(behavior) as Box<dyn Behavior<_>>]);

                assert!(!context.view_model.borrow().snap_to_grid);

                sender
                    .send(UpdateSnapToGridCommand { value: true })
                    .expect("send should succeed");

                let updated = context.wait_until(Duration::from_secs(1), || {
                    context.view_model.borrow().snap_to_grid
                        && context
                            .read_global_state(|state| state.choreography.settings.snap_to_grid)
                });
                assert!(updated);
                assert!(preferences.get_bool(SettingsPreferenceKeys::SNAP_TO_GRID, false));
                assert!(context.redraw_receiver.try_recv().is_ok());
            },
        );
    });

    let report = choreography_settings::run_suite(&suite);
    assert!(report.is_success());
}
