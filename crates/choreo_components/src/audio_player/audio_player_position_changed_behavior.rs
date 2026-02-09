use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use crossbeam_channel::Sender;
use nject::injectable;
use slint::TimerMode;

use crate::behavior::{Behavior, CompositeDisposable, TimerDisposable};
use crate::logging::BehaviorLog;

use super::audio_player_view_model::AudioPlayerViewModel;
use super::messages::AudioPlayerPositionChangedEvent;

#[injectable]
#[inject(|publishers: Vec<Sender<AudioPlayerPositionChangedEvent>>| Self::new(publishers))]
pub struct AudioPlayerPositionChangedBehavior {
    publishers: Vec<Sender<AudioPlayerPositionChangedEvent>>,
}

impl AudioPlayerPositionChangedBehavior {
    pub fn new(publishers: Vec<Sender<AudioPlayerPositionChangedEvent>>) -> Self {
        Self { publishers }
    }
}

impl Behavior<AudioPlayerViewModel> for AudioPlayerPositionChangedBehavior {
    fn activate(
        &self,
        view_model: &mut AudioPlayerViewModel,
        disposables: &mut CompositeDisposable,
    ) {
        BehaviorLog::behavior_activated(
            "AudioPlayerPositionChangedBehavior",
            "AudioPlayerViewModel",
        );
        let Some(view_model_handle) = view_model.self_handle().and_then(|handle| handle.upgrade())
        else {
            return;
        };
        let publishers = self.publishers.clone();
        let last_position = Rc::new(RefCell::new(None::<f64>));
        let timer = slint::Timer::default();
        timer.start(TimerMode::Repeated, Duration::from_millis(16), move || {
            let view_model = view_model_handle.borrow();
            let position = view_model.position;
            let mut last_position = last_position.borrow_mut();
            let should_publish = match *last_position {
                Some(previous) => (previous - position).abs() > 0.0001,
                None => true,
            };
            if should_publish {
                *last_position = Some(position);
                for publisher in &publishers {
                    let _ = publisher.try_send(AudioPlayerPositionChangedEvent {
                        position_seconds: position,
                    });
                }
            }
        });
        disposables.add(Box::new(TimerDisposable::new(timer)));
    }
}
