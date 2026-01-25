use crossbeam_channel::Receiver;

use crate::behavior::{Behavior, CompositeDisposable};
use crate::logging::BehaviorLog;

use super::audio_player_view_model::AudioPlayerViewModel;
use super::messages::CloseAudioFileCommand;

pub struct CloseAudioFileBehavior {
    receiver: Receiver<CloseAudioFileCommand>,
}

impl CloseAudioFileBehavior {
    pub fn new(receiver: Receiver<CloseAudioFileCommand>) -> Self {
        Self { receiver }
    }

    pub fn try_handle(&self, view_model: &mut AudioPlayerViewModel) -> bool {
        match self.receiver.try_recv() {
            Ok(_) => {
                Self::handle_close(view_model);
                true
            }
            Err(_) => false,
        }
    }

    fn handle_close(view_model: &mut AudioPlayerViewModel) {
        view_model.player = None;
        view_model.stream_factory = None;
        view_model.title = "Audio".to_string();
        view_model.position = 0.0;
        view_model.duration = 0.0;
        view_model.is_playing = false;
        view_model.can_seek = false;
        view_model.can_set_speed = false;
    }
}

impl Behavior<AudioPlayerViewModel> for CloseAudioFileBehavior {
    fn activate(&self, _view_model: &mut AudioPlayerViewModel, _disposables: &mut CompositeDisposable) {
        BehaviorLog::behavior_activated("CloseAudioFileBehavior", "AudioPlayerViewModel");
    }
}
