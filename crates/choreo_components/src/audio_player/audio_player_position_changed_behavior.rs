use crossbeam_channel::Sender;
use nject::injectable;

use crate::behavior::{Behavior, CompositeDisposable};
use crate::logging::BehaviorLog;

use super::audio_player_view_model::AudioPlayerViewModel;
use super::messages::AudioPlayerPositionChangedEvent;

#[injectable]
#[inject(|publisher: Sender<AudioPlayerPositionChangedEvent>| Self::new(publisher))]
pub struct AudioPlayerPositionChangedBehavior {
    publisher: Sender<AudioPlayerPositionChangedEvent>,
}

impl AudioPlayerPositionChangedBehavior {
    pub fn new(publisher: Sender<AudioPlayerPositionChangedEvent>) -> Self {
        Self { publisher }
    }

    pub fn publish(&self, position_seconds: f64) {
        let _ = self
            .publisher
            .send(AudioPlayerPositionChangedEvent { position_seconds });
    }
}

impl Behavior<AudioPlayerViewModel> for AudioPlayerPositionChangedBehavior {
    fn activate(&self, _view_model: &mut AudioPlayerViewModel, _disposables: &mut CompositeDisposable) {
        BehaviorLog::behavior_activated(
            "AudioPlayerPositionChangedBehavior",
            "AudioPlayerViewModel",
        );
    }
}
