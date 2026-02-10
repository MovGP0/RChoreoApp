use std::cell::RefCell;
use std::rc::Rc;

use crate::audio_player;

use audio_player::Report;
use choreo_components::audio_player::AudioPlayer;
use audio_player::TestAudioPlayer;
use audio_player::TestAudioPlayerState;
use choreo_components::audio_player::AudioPlayerViewState;

#[derive(Default)]
struct LaggingAudioPlayer {
    is_playing: bool,
    can_seek: bool,
    current_position: f64,
    last_seek: Option<f64>,
}

impl AudioPlayer for LaggingAudioPlayer {
    fn is_playing(&self) -> bool {
        self.is_playing
    }

    fn can_seek(&self) -> bool {
        self.can_seek
    }

    fn can_set_speed(&self) -> bool {
        true
    }

    fn duration(&self) -> f64 {
        30.0
    }

    fn current_position(&self) -> f64 {
        self.current_position
    }

    fn play(&mut self) {
        self.is_playing = true;
    }

    fn pause(&mut self) {
        self.is_playing = false;
    }

    fn stop(&mut self) {
        self.is_playing = false;
        self.current_position = 0.0;
    }

    fn seek(&mut self, position: f64) {
        // Simulate async actor: command accepted, reported position still stale.
        self.last_seek = Some(position);
    }

    fn set_speed(&mut self, _speed: f64) {}

    fn set_volume(&mut self, _volume: f64) {}

    fn set_balance(&mut self, _balance: f64) {}

    fn set_loop(&mut self, _loop_enabled: bool) {}
}

#[test]
#[serial_test::serial]
fn audio_player_view_state_spec() {
    let suite = rspec::describe("audio player view state", (), |spec| {
        spec.it(
            "restores playback after drag when view state was playing before drag",
            |_| {
                let context = audio_player::AudioPlayerTestContext::new(vec![]);
                let player_state = Rc::new(RefCell::new(TestAudioPlayerState::default()));
                context
                    .view_model
                    .borrow_mut()
                    .set_player(Box::new(TestAudioPlayer {
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

        spec.it(
            "ignores stale agent position until dragged target is acknowledged",
            |_| {
                let context = audio_player::AudioPlayerTestContext::new(vec![]);
                let player_state = Rc::new(RefCell::new(TestAudioPlayerState::default()));
                context
                    .view_model
                    .borrow_mut()
                    .set_player(Box::new(TestAudioPlayer {
                        state: Rc::clone(&player_state),
                    }));

                let mut view_state = AudioPlayerViewState::new();

                {
                    let mut view_model = context.view_model.borrow_mut();
                    view_model.is_playing = false;
                    view_state.on_position_drag_started(&mut view_model);
                    view_state.on_position_drag_completed(&mut view_model, 18.0);
                }

                assert!(!view_state.should_accept_player_position(4.0));
                assert!(view_state.should_accept_player_position(18.0));
            },
        );

        spec.it(
            "keeps dragged position after commit when player reports stale current position",
            |_| {
                let context = audio_player::AudioPlayerTestContext::new(vec![]);
                let mut lagging = LaggingAudioPlayer {
                    can_seek: true,
                    current_position: 4.0,
                    ..LaggingAudioPlayer::default()
                };
                lagging.is_playing = false;
                context.view_model.borrow_mut().set_player(Box::new(lagging));

                let mut view_state = AudioPlayerViewState::new();
                {
                    let mut view_model = context.view_model.borrow_mut();
                    view_model.is_playing = false;
                    view_state.on_position_drag_started(&mut view_model);
                    view_state.on_position_drag_completed(&mut view_model, 18.0);
                    assert_eq!(view_model.position, 18.0);
                }
            },
        );
    });

    let report = audio_player::run_suite(&suite);
    assert!(report.is_success());
}
