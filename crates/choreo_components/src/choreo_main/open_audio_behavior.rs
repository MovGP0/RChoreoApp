use crossbeam_channel::Sender;
use nject::injectable;

use crate::audio_player::OpenAudioFileCommand;

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
