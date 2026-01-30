use crossbeam_channel::Sender;
use nject::injectable;

use crate::audio_player::OpenAudioFileCommand;
use crate::behavior::{Behavior, CompositeDisposable};
use crate::logging::BehaviorLog;

use super::main_view_model::MainViewModel;

#[injectable]
#[inject(|sender: Sender<OpenAudioFileCommand>| Self { sender })]
pub struct OpenAudioBehavior {
    sender: Sender<OpenAudioFileCommand>,
}

impl OpenAudioBehavior {
    pub fn new(sender: Sender<OpenAudioFileCommand>) -> Self {
        Self { sender }
    }

    pub fn open_audio(&self, path: String) {
        let _ = self.sender.send(OpenAudioFileCommand { file_path: path });
    }
}

impl Behavior<MainViewModel> for OpenAudioBehavior {
    fn activate(&self, _view_model: &mut MainViewModel, _disposables: &mut CompositeDisposable) {
        BehaviorLog::behavior_activated("OpenAudioBehavior", "MainViewModel");
    }
}
