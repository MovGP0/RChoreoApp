use crossbeam_channel::Sender;

use crate::audio_player::OpenAudioFileCommand;

use super::main_view_model::MainViewModel;

pub struct OpenAudioBehavior {
    sender: Sender<OpenAudioFileCommand>,
}

impl OpenAudioBehavior {
    pub fn new(sender: Sender<OpenAudioFileCommand>) -> Self {
        Self { sender }
    }

    pub fn open_audio(&self, view_model: &mut MainViewModel, path: String) {
        let _ = self.sender.send(OpenAudioFileCommand { file_path: path });
        view_model.is_audio_player_open = true;
    }
}
