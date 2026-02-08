use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use crate::audio_player;

use audio_player::Report;
use choreo_components::audio_player::AudioPlayerBehavior;
use choreo_components::behavior::Behavior;

#[test]
#[serial_test::serial]
fn audio_player_behavior_spec() {
    let suite = rspec::describe("audio player behavior", (), |spec| {
        spec.it("syncs view model state from player on timer", |_| {
            let context = audio_player::AudioPlayerTestContext::new(vec![
                Box::new(AudioPlayerBehavior) as Box<dyn Behavior<_>>,
            ]);

            let player_state = Rc::new(RefCell::new(audio_player::TestAudioPlayerState {
                is_playing: true,
                duration: 123.0,
                current_position: 7.5,
                ..Default::default()
            }));
            context.view_model.borrow_mut().set_player(Box::new(audio_player::TestAudioPlayer {
                state: player_state,
            }));

            let synced = context.wait_until(Duration::from_secs(1), || {
                let vm = context.view_model.borrow();
                (vm.duration - 123.0).abs() < 0.0001
                    && (vm.position - 7.5).abs() < 0.0001
                    && vm.is_playing
            });
            assert!(synced);
        });
    });

    let report = audio_player::run_suite(&suite);
    assert!(report.is_success());
}
