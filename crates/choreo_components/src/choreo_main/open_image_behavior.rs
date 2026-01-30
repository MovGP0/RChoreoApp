use std::time::Duration;

use crossbeam_channel::{Receiver, Sender};
use nject::injectable;
use slint::TimerMode;

use crate::behavior::{Behavior, CompositeDisposable};
use crate::behavior::TimerDisposable;
use crate::logging::BehaviorLog;

use super::main_view_model::MainViewModel;
use super::messages::OpenSvgFileCommand;

#[derive(Clone)]
#[injectable]
#[inject(
    |sender: Sender<OpenSvgFileCommand>, receiver: Receiver<String>| {
        Self { sender, receiver }
    }
)]
pub struct OpenImageBehavior {
    sender: Sender<OpenSvgFileCommand>,
    receiver: Receiver<String>,
}

impl OpenImageBehavior {
    pub fn new(sender: Sender<OpenSvgFileCommand>, receiver: Receiver<String>) -> Self {
        Self { sender, receiver }
    }

    fn handle_open_svg(&self, path: String) {
        let _ = self.sender.send(OpenSvgFileCommand { file_path: path });
    }
}

impl Behavior<MainViewModel> for OpenImageBehavior {
    fn activate(&self, _view_model: &mut MainViewModel, disposables: &mut CompositeDisposable) {
        BehaviorLog::behavior_activated("OpenImageBehavior", "MainViewModel");
        let receiver = self.receiver.clone();
        let behavior = self.clone();
        let timer = slint::Timer::default();
        timer.start(TimerMode::Repeated, Duration::from_millis(16), move || {
            while let Ok(path) = receiver.try_recv() {
                behavior.handle_open_svg(path);
            }
        });
        disposables.add(Box::new(TimerDisposable::new(timer)));
    }
}
