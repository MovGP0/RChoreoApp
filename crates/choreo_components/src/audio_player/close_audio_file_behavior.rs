use std::time::Duration;

use crossbeam_channel::Receiver;
use nject::injectable;
use slint::TimerMode;

use crate::behavior::{Behavior, CompositeDisposable, TimerDisposable};
use crate::logging::BehaviorLog;

use super::audio_player_view_model::AudioPlayerViewModel;
use super::messages::CloseAudioFileCommand;

#[injectable]
#[inject(|receiver: Receiver<CloseAudioFileCommand>| Self::new(receiver))]
pub struct CloseAudioFileBehavior {
    receiver: Receiver<CloseAudioFileCommand>,
}

impl CloseAudioFileBehavior {
    pub fn new(receiver: Receiver<CloseAudioFileCommand>) -> Self
    {
        Self { receiver }
    }
}

impl Behavior<AudioPlayerViewModel> for CloseAudioFileBehavior {
    fn activate(&self, view_model: &mut AudioPlayerViewModel, disposables: &mut CompositeDisposable)
    {
        BehaviorLog::behavior_activated("CloseAudioFileBehavior", "AudioPlayerViewModel");
        let Some(view_model_handle) = view_model.self_handle().and_then(|handle| handle.upgrade())
        else {
            return;
        };
        let receiver = self.receiver.clone();
        let timer = slint::Timer::default();
        timer.start(TimerMode::Repeated, Duration::from_millis(16), move || {
            while receiver.try_recv().is_ok() {
                let mut view_model = view_model_handle.borrow_mut();
                view_model.player = None;
                view_model.stream_factory = None;
                view_model.position = 0.0;
                view_model.duration = 0.0;
                view_model.is_playing = false;
                view_model.can_seek = false;
                view_model.can_set_speed = false;
            }
        });
        disposables.add(Box::new(TimerDisposable::new(timer)));
    }
}
