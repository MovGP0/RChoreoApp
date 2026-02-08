use std::time::Duration;

use crate::scenes;

use choreo_components::behavior::Behavior;
use choreo_components::choreography_settings::ShowTimestampsChangedEvent;
use choreo_components::scenes::ShowSceneTimestampsBehavior;
use crossbeam_channel::unbounded;
use scenes::Report;

#[test]
#[serial_test::serial]
fn show_scene_timestamps_behavior_spec() {
    let suite = rspec::describe("show scene timestamps behavior", (), |spec| {
        spec.it(
            "initializes from choreography settings on activation",
            |_| {
                let context = scenes::ScenesTestContext::new();
                context.update_global_state(|state| {
                    state.choreography.settings.show_timestamps = true;
                });

                let (_sender, receiver) = unbounded::<ShowTimestampsChangedEvent>();
                let behavior =
                    ShowSceneTimestampsBehavior::new(context.global_state_store.clone(), receiver);
                context.activate_behaviors(vec![Box::new(behavior) as Box<dyn Behavior<_>>]);

                assert!(context.view_model.borrow().show_timestamps);
            },
        );

        spec.it("updates view model and global state when toggled", |_| {
            let context = scenes::ScenesTestContext::new();
            context.update_global_state(|state| {
                state.choreography.settings.show_timestamps = false;
            });

            let (sender, receiver) = unbounded::<ShowTimestampsChangedEvent>();
            let behavior =
                ShowSceneTimestampsBehavior::new(context.global_state_store.clone(), receiver);
            context.activate_behaviors(vec![Box::new(behavior) as Box<dyn Behavior<_>>]);

            sender
                .send(ShowTimestampsChangedEvent { is_enabled: true })
                .expect("send should succeed");

            let applied = context.wait_until(Duration::from_secs(1), || {
                context.view_model.borrow().show_timestamps
                    && context
                        .read_global_state(|state| state.choreography.settings.show_timestamps)
            });
            assert!(applied);
        });
    });

    let report = scenes::run_suite(&suite);
    assert!(report.is_success());
}
