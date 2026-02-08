use std::time::Duration;

use crate::choreography_settings;

use choreo_components::behavior::Behavior;
use choreo_components::choreography_settings::LoadChoreographySettingsBehavior;
use choreo_components::choreography_settings::ReloadChoreographySettingsCommand;
use crossbeam_channel::unbounded;
use choreography_settings::Report;

#[test]
#[serial_test::serial]
fn load_choreography_settings_behavior_spec() {
    let suite = rspec::describe("load choreography settings behavior", (), |spec| {
        spec.it("maps choreography fields into view model on activation", |_| {
            let context = choreography_settings::ChoreographySettingsTestContext::new();
            context.update_global_state(|state| {
                state.choreography.name = "My Choreo".to_string();
                state.choreography.author = Some("Author".to_string());
                state.choreography.floor.size_front = 12;
                state.choreography.settings.show_timestamps = true;
            });

            let (_reload_sender, reload_receiver) = unbounded::<ReloadChoreographySettingsCommand>();
            let behavior = LoadChoreographySettingsBehavior::new_with_receiver(
                context.global_state_store.clone(),
                reload_receiver,
            );
            context.activate_behaviors(vec![Box::new(behavior) as Box<dyn Behavior<_>>]);

            let vm = context.view_model.borrow();
            assert_eq!(vm.name, "My Choreo");
            assert_eq!(vm.author, "Author");
            assert_eq!(vm.floor_front, 12);
            assert!(vm.show_timestamps);
        });

        spec.it("reloads view model when reload command is received", |_| {
            let context = choreography_settings::ChoreographySettingsTestContext::new();
            context.update_global_state(|state| {
                state.choreography.name = "Before".to_string();
            });

            let (reload_sender, reload_receiver) = unbounded::<ReloadChoreographySettingsCommand>();
            let behavior = LoadChoreographySettingsBehavior::new_with_receiver(
                context.global_state_store.clone(),
                reload_receiver,
            );
            context.activate_behaviors(vec![Box::new(behavior) as Box<dyn Behavior<_>>]);

            context.update_global_state(|state| {
                state.choreography.name = "After".to_string();
                state.choreography.floor.size_back = 77;
            });
            reload_sender
                .send(ReloadChoreographySettingsCommand)
                .expect("reload send should succeed");

            let updated = context.wait_until(Duration::from_secs(1), || {
                let vm = context.view_model.borrow();
                vm.name == "After" && vm.floor_back == 77
            });
            assert!(updated);
        });
    });

    let report = choreography_settings::run_suite(&suite);
    assert!(report.is_success());
}
