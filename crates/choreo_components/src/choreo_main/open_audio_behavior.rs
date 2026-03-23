use crossbeam_channel::Sender;
use nject::injectable;

use crate::audio_player::OpenAudioFileCommand;
use crate::behavior::Behavior;
use crate::behavior::CompositeDisposable;
use crate::logging::BehaviorLog;

use super::main_view_model::MainViewModel;
use super::messages::OpenAudioRequested;

#[injectable]
#[inject(|sender: Sender<OpenAudioFileCommand>| Self::new(sender))]
#[derive(Clone)]
pub struct OpenAudioBehavior {
    sender: Sender<OpenAudioFileCommand>,
}

impl OpenAudioBehavior {
    pub fn new(sender: Sender<OpenAudioFileCommand>) -> Self {
        Self { sender }
    }

    pub fn apply(&self, request: OpenAudioRequested) {
        let _ = self.sender.try_send(OpenAudioFileCommand {
            file_path: request.file_path,
            trace_context: request.trace_context,
        });
    }
}

impl Behavior<MainViewModel> for OpenAudioBehavior {
    fn activate(&self, _view_model: &mut MainViewModel, _disposables: &mut CompositeDisposable) {
        BehaviorLog::behavior_activated("OpenAudioBehavior", "MainViewModel");
    }
}
