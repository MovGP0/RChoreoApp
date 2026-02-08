use std::time::Duration;

use crate::audio_player;

use audio_player::Report;
use choreo_components::audio_player::AudioPlayerPositionChangedBehavior;
use choreo_components::behavior::Behavior;
use crossbeam_channel::bounded;

#[test]
#[serial_test::serial]
fn audio_player_position_changed_behavior_spec() {
    let suite = rspec::describe("audio player position changed behavior", (), |spec| {
        spec.it("publishes when position changes", |_| {
            let (sender, receiver) = bounded(16);
            let behavior = AudioPlayerPositionChangedBehavior::new(sender);
            let context = audio_player::AudioPlayerTestContext::new(vec![
                Box::new(behavior) as Box<dyn Behavior<_>>,
            ]);

            context.view_model.borrow_mut().position = 1.0;
            let published = context.wait_until(Duration::from_secs(1), || receiver.try_recv().is_ok());
            assert!(published);

            context.view_model.borrow_mut().position = 2.0;
            let published_again = context.wait_until(Duration::from_secs(1), || receiver.try_recv().is_ok());
            assert!(published_again);
        });
    });

    let report = audio_player::run_suite(&suite);
    assert!(report.is_success());
}
