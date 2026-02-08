use std::time::Duration;

use crate::choreography_settings;

use choreo_components::behavior::Behavior;
use choreo_components::choreography_settings::UpdateDrawPathFromBehavior;
use choreo_components::choreography_settings::UpdateDrawPathFromCommand;
use choreo_components::preferences::Preferences;
use choreo_models::SettingsPreferenceKeys;
use crossbeam_channel::unbounded;
use choreography_settings::Report;

#[test]
#[serial_test::serial]
fn update_draw_path_from_behavior_spec() {
    let suite = rspec::describe("update draw path from behavior", (), |spec| {
        spec.it("initializes from preferences and updates value with redraw", |_| {
            let preferences = choreo_components::preferences::InMemoryPreferences::new();
            preferences.set_bool(SettingsPreferenceKeys::DRAW_PATH_FROM, true);

            let (redraw_sender, redraw_receiver) = unbounded();
            let context = choreography_settings::ChoreographySettingsTestContext::with_redraw_receiver(redraw_receiver);
            let (sender, receiver) = unbounded::<UpdateDrawPathFromCommand>();
            let behavior = UpdateDrawPathFromBehavior::new_with_receiver(
                preferences.clone(),
                redraw_sender,
                receiver,
            );
            context.activate_behaviors(vec![Box::new(behavior) as Box<dyn Behavior<_>>]);

            assert!(context.view_model.borrow().draw_path_from);

            sender
                .send(UpdateDrawPathFromCommand { value: false })
                .expect("send should succeed");

            let updated = context.wait_until(Duration::from_secs(1), || {
                !context.view_model.borrow().draw_path_from
            });
            assert!(updated);
            assert!(!preferences.get_bool(SettingsPreferenceKeys::DRAW_PATH_FROM, true));
            assert!(context.redraw_receiver.try_recv().is_ok());
        });
    });

    let report = choreography_settings::run_suite(&suite);
    assert!(report.is_success());
}
