use crossbeam_channel::Sender;

use crate::audio_player::OpenAudioFileCommand;

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
