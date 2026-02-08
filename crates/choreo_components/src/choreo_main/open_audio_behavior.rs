use std::time::Duration;

use crossbeam_channel::{Receiver, Sender};
use nject::injectable;
use slint::TimerMode;

use crate::audio_player::OpenAudioFileCommand;
use crate::behavior::TimerDisposable;
use crate::behavior::{Behavior, CompositeDisposable};
use crate::logging::BehaviorLog;

use super::main_view_model::MainViewModel;
use super::messages::OpenAudioRequested;

#[derive(Clone)]
#[injectable]
#[inject(
    |sender: Sender<OpenAudioFileCommand>, receiver: Receiver<OpenAudioRequested>| {
        Self { sender, receiver }
    }
)]
pub struct OpenAudioBehavior {
    sender: Sender<OpenAudioFileCommand>,
    receiver: Receiver<OpenAudioRequested>,
}

impl OpenAudioBehavior {
    pub fn new(
        sender: Sender<OpenAudioFileCommand>,
        receiver: Receiver<OpenAudioRequested>,
    ) -> Self {
        Self { sender, receiver }
    }

    fn handle_open_audio(&self, command: OpenAudioRequested) {
        let _ = self.sender.try_send(OpenAudioFileCommand {
            file_path: command.file_path,
        });
    }
}

impl Behavior<MainViewModel> for OpenAudioBehavior {
    fn activate(&self, _view_model: &mut MainViewModel, disposables: &mut CompositeDisposable) {
        BehaviorLog::behavior_activated("OpenAudioBehavior", "MainViewModel");
        let receiver = self.receiver.clone();
        let behavior = self.clone();
        let timer = slint::Timer::default();
        timer.start(TimerMode::Repeated, Duration::from_millis(16), move || {
            while let Ok(command) = receiver.try_recv() {
                behavior.handle_open_audio(command);
            }
        });
        disposables.add(Box::new(TimerDisposable::new(timer)));
    }
}
