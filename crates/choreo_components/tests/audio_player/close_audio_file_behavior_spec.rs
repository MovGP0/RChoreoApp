use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use crate::audio_player;

use audio_player::Report;
use choreo_components::audio_player::CloseAudioFileBehavior;
use choreo_components::audio_player::CloseAudioFileCommand;
use choreo_components::behavior::Behavior;
use crossbeam_channel::unbounded;

#[test]
#[serial_test::serial]
fn close_audio_file_behavior_spec() {
    let suite = rspec::describe("close audio file behavior", (), |spec| {
        spec.it("resets player state when close command is received", |_| {
            let (sender, receiver) = unbounded::<CloseAudioFileCommand>();
            let behavior = CloseAudioFileBehavior::new(receiver);
            let context = audio_player::AudioPlayerTestContext::new(vec![
                Box::new(behavior) as Box<dyn Behavior<_>>
            ]);

            let player_state = Rc::new(RefCell::new(audio_player::TestAudioPlayerState {
                is_playing: true,
                duration: 40.0,
                current_position: 5.0,
                ..Default::default()
            }));
            {
                let mut vm = context.view_model.borrow_mut();
                vm.set_player(Box::new(audio_player::TestAudioPlayer {
                    state: player_state,
                }));
                vm.position = 5.0;
                vm.duration = 40.0;
                vm.is_playing = true;
                vm.can_seek = true;
                vm.can_set_speed = true;
                vm.stream_factory = Some(Box::new(|| {
                    Ok(Box::new(std::io::Cursor::new(vec![1, 2, 3])))
                }));
            }

            sender
                .send(CloseAudioFileCommand)
                .expect("send should succeed");

            let reset = context.wait_until(Duration::from_secs(1), || {
                let vm = context.view_model.borrow();
                vm.player.is_none()
                    && vm.stream_factory.is_none()
                    && vm.position == 0.0
                    && vm.duration == 0.0
                    && !vm.is_playing
                    && !vm.can_seek
                    && !vm.can_set_speed
            });
            assert!(reset);
        });
    });

    let report = audio_player::run_suite(&suite);
    assert!(report.is_success());
}
