use std::time::Duration;

use crossbeam_channel::Receiver;
use nject::injectable;
use slint::TimerMode;

use crate::behavior::{Behavior, CompositeDisposable, TimerDisposable};
use crate::logging::BehaviorLog;
use crate::observability::start_internal_span;

use super::audio_player_view_model::AudioPlayerViewModel;
use super::messages::CloseAudioFileCommand;

#[injectable]
#[inject(|receiver: Receiver<CloseAudioFileCommand>| Self::new(receiver))]
pub struct CloseAudioFileBehavior {
    receiver: Receiver<CloseAudioFileCommand>,
}

impl CloseAudioFileBehavior {
    pub fn new(receiver: Receiver<CloseAudioFileCommand>) -> Self {
        Self { receiver }
    }
}

impl Behavior<AudioPlayerViewModel> for CloseAudioFileBehavior {
    fn activate(
        &self,
        view_model: &mut AudioPlayerViewModel,
        disposables: &mut CompositeDisposable,
    ) {
        BehaviorLog::behavior_activated("CloseAudioFileBehavior", "AudioPlayerViewModel");
        let Some(view_model_handle) = view_model.self_handle().and_then(|handle| handle.upgrade())
        else {
            return;
        };
        let receiver = self.receiver.clone();
        let timer = slint::Timer::default();
        timer.start(TimerMode::Repeated, Duration::from_millis(16), move || {
            while let Ok(command) = receiver.try_recv() {
                let mut span = start_internal_span(
                    "audio_player.close_audio_file",
                    command.trace_context.as_ref(),
                );
                span.set_string_attribute(
                    "choreo.command.type",
                    "CloseAudioFileCommand".to_string(),
                );

                let Ok(mut view_model) = view_model_handle.try_borrow_mut() else {
                    span.set_bool_attribute("choreo.success", false);
                    continue;
                };
                view_model.player = None;
                view_model.stream_factory = None;
                view_model.position = 0.0;
                view_model.duration = 0.0;
                view_model.is_playing = false;
                view_model.can_seek = false;
                view_model.can_set_speed = false;
                span.set_bool_attribute("choreo.success", true);
            }
        });
        disposables.add(Box::new(TimerDisposable::new(timer)));
    }
}
