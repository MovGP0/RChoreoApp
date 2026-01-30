use std::time::Duration;

use crossbeam_channel::{Receiver, Sender};
use nject::injectable;
use slint::TimerMode;

use crate::audio_player::OpenAudioFileCommand;
use crate::behavior::{Behavior, CompositeDisposable};
use crate::behavior::TimerDisposable;
use crate::logging::BehaviorLog;

use super::main_view_model::MainViewModel;

#[derive(Clone)]
#[injectable]
#[inject(
    |sender: Sender<OpenAudioFileCommand>, receiver: Receiver<String>| {
        Self { sender, receiver }
    }
)]
pub struct OpenAudioBehavior {
    sender: Sender<OpenAudioFileCommand>,
    receiver: Receiver<String>,
}

impl OpenAudioBehavior {
    pub fn new(sender: Sender<OpenAudioFileCommand>, receiver: Receiver<String>) -> Self {
        Self { sender, receiver }
    }

    fn handle_open_audio(&self, path: String) {
        let _ = self.sender.send(OpenAudioFileCommand { file_path: path });
    }
}

impl Behavior<MainViewModel> for OpenAudioBehavior {
    fn activate(&self, _view_model: &mut MainViewModel, disposables: &mut CompositeDisposable) {
        BehaviorLog::behavior_activated("OpenAudioBehavior", "MainViewModel");
        let receiver = self.receiver.clone();
        let behavior = self.clone();
        let timer = slint::Timer::default();
        timer.start(TimerMode::Repeated, Duration::from_millis(16), move || {
            while let Ok(path) = receiver.try_recv() {
                behavior.handle_open_audio(path);
            }
        });
        disposables.add(Box::new(TimerDisposable::new(timer)));
    }
}
