use std::time::Duration;

use nject::injectable;
use slint::TimerMode;

use crate::behavior::{Behavior, CompositeDisposable, TimerDisposable};
use crate::logging::BehaviorLog;

use super::audio_player_view_model::AudioPlayerViewModel;

#[injectable]
#[inject(|| Self)]
pub struct AudioPlayerBehavior;

impl Behavior<AudioPlayerViewModel> for AudioPlayerBehavior {
    fn activate(&self, view_model: &mut AudioPlayerViewModel, disposables: &mut CompositeDisposable)
    {
        BehaviorLog::behavior_activated("AudioPlayerBehavior", "AudioPlayerViewModel");
        let Some(view_model_handle) = view_model.self_handle().and_then(|handle| handle.upgrade())
        else {
            return;
        };

        let timer = slint::Timer::default();
        timer.start(TimerMode::Repeated, Duration::from_millis(16), move || {
            let mut view_model = view_model_handle.borrow_mut();
            view_model.sync_from_player();
        });
        disposables.add(Box::new(TimerDisposable::new(timer)));
    }
}
