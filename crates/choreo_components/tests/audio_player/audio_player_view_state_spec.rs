use std::cell::RefCell;
use std::rc::Rc;

use crate::audio_player;

use audio_player::Report;
use audio_player::TestAudioPlayer;
use audio_player::TestAudioPlayerState;
use choreo_components::audio_player::AudioPlayerViewState;

#[test]
#[serial_test::serial]
fn audio_player_view_state_spec() {
    let suite = rspec::describe("audio player view state", (), |spec| {
        spec.it(
            "restores playback after drag when view state was playing before drag",
            |_| {
                let context = audio_player::AudioPlayerTestContext::new(vec![]);
                let player_state = Rc::new(RefCell::new(TestAudioPlayerState::default()));
                context.view_model.borrow_mut().set_player(Box::new(TestAudioPlayer {
                    state: Rc::clone(&player_state),
                }));

                let mut view_state = AudioPlayerViewState::new();

                {
                    let mut view_model = context.view_model.borrow_mut();
                    view_model.is_playing = true;
                    view_state.on_position_drag_started(&mut view_model);
                    view_model.position = 12.0;
                    view_state.on_position_drag_completed(&mut view_model, 12.0);
                }

                let state = player_state.borrow();
                assert_eq!(state.last_seek, Some(12.0));
                assert!(state.is_playing);
            },
        );
    });

    let report = audio_player::run_suite(&suite);
    assert!(report.is_success());
}
