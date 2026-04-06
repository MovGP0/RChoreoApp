use crossbeam_channel::Sender;
use nject::injectable;

use crate::audio_player::OpenAudioFileCommand;

use super::actions::OpenAudioRequested;
use super::state::ChoreoMainState;
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

pub(super) fn request_open_audio(state: &mut ChoreoMainState, request: OpenAudioRequested) {
    state.outgoing_audio_requests.push(request);
}
