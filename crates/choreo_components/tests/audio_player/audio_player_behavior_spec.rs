use crate::audio_player;
use std::cell::RefCell;
use std::rc::Rc;

use audio_player::Report;
use choreo_components::audio_player::AudioPlayerBehavior;
use choreo_components::behavior::Behavior;

#[test]
#[serial_test::serial]
fn audio_player_behavior_spec() {
    let suite = rspec::describe("audio player behavior", (), |spec| {
        spec.it(
            "can be activated without mutating player state in callbacks",
            |_| {
                let context =
                    audio_player::AudioPlayerTestContext::new(vec![
                        Box::new(AudioPlayerBehavior) as Box<dyn Behavior<_>>
                    ]);

                let player_state = Rc::new(RefCell::new(audio_player::TestAudioPlayerState {
                    is_playing: true,
                    duration: 123.0,
                    current_position: 7.5,
                    ..Default::default()
                }));
                context.view_model.borrow_mut().set_player(Box::new(
                    audio_player::TestAudioPlayer {
                        state: player_state,
                    },
                ));
                context.view_model.borrow_mut().sync_from_player();

                context.pump_events();
                let vm = context.view_model.borrow();
                assert!((vm.duration - 123.0).abs() < 0.0001);
                assert!((vm.position - 7.5).abs() < 0.0001);
                assert!(vm.is_playing);
            },
        );
    });

    let report = audio_player::run_suite(&suite);
    assert!(report.is_success());
}
