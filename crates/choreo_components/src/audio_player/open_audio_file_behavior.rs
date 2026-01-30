use std::io;
use crossbeam_channel::Receiver;
use choreo_models::SettingsPreferenceKeys;
use nject::injectable;

use crate::behavior::{Behavior, CompositeDisposable};
use crate::logging::BehaviorLog;
use crate::preferences::Preferences;

use super::audio_player_view_model::AudioPlayerViewModel;
use super::messages::OpenAudioFileCommand;

#[injectable]
#[inject(|receiver: Receiver<OpenAudioFileCommand>, preferences: P| Self::new(receiver, preferences))]
pub struct OpenAudioFileBehavior<P: Preferences> {
    receiver: Receiver<OpenAudioFileCommand>,
    preferences: P,
}

impl<P: Preferences> OpenAudioFileBehavior<P> {
    pub fn new(receiver: Receiver<OpenAudioFileCommand>, preferences: P) -> Self {
        Self {
            receiver,
            preferences,
        }
    }

    pub fn try_handle(&self, view_model: &mut AudioPlayerViewModel) -> bool {
        match self.receiver.try_recv() {
            Ok(command) => {
                self.handle_open(view_model, command);
                true
            }
            Err(_) => false,
        }
    }

    fn handle_open(&self, view_model: &mut AudioPlayerViewModel, command: OpenAudioFileCommand) {
        if command.file_path.trim().is_empty() {
            return;
        }

        let file_path = command.file_path;
        let stream_path = file_path.clone();
        view_model.stream_factory = Some(Box::new(move || {
            let file = std::fs::File::open(&stream_path)?;
            Ok(Box::new(file) as Box<dyn io::Read + Send>)
        }));

        self.preferences
            .set_string(SettingsPreferenceKeys::LAST_OPENED_AUDIO_FILE, file_path);
    }
}

impl<P: Preferences> Behavior<AudioPlayerViewModel> for OpenAudioFileBehavior<P> {
    fn activate(&self, _view_model: &mut AudioPlayerViewModel, _disposables: &mut CompositeDisposable) {
        BehaviorLog::behavior_activated("OpenAudioFileBehavior", "AudioPlayerViewModel");
    }
}
