use std::time::Duration;

use crossbeam_channel::{Receiver, Sender};
use nject::injectable;
use slint::TimerMode;

use crate::behavior::TimerDisposable;
use crate::behavior::{Behavior, CompositeDisposable};
use crate::logging::BehaviorLog;

use super::main_view_model::MainViewModel;
use super::messages::{OpenImageRequested, OpenSvgFileCommand};

#[derive(Clone)]
#[injectable]
#[inject(
    |sender: Sender<OpenSvgFileCommand>, receiver: Receiver<OpenImageRequested>| {
        Self { sender, receiver }
    }
)]
pub struct OpenImageBehavior {
    sender: Sender<OpenSvgFileCommand>,
    receiver: Receiver<OpenImageRequested>,
}

impl OpenImageBehavior {
    pub fn new(sender: Sender<OpenSvgFileCommand>, receiver: Receiver<OpenImageRequested>) -> Self {
        Self { sender, receiver }
    }

    fn handle_open_svg(&self, command: OpenImageRequested) {
        let _ = self.sender.send(OpenSvgFileCommand {
            file_path: command.file_path,
        });
    }
}

impl Behavior<MainViewModel> for OpenImageBehavior {
    fn activate(&self, _view_model: &mut MainViewModel, disposables: &mut CompositeDisposable) {
        BehaviorLog::behavior_activated("OpenImageBehavior", "MainViewModel");
        let receiver = self.receiver.clone();
        let behavior = self.clone();
        let timer = slint::Timer::default();
        timer.start(TimerMode::Repeated, Duration::from_millis(16), move || {
            while let Ok(command) = receiver.try_recv() {
                behavior.handle_open_svg(command);
            }
        });
        disposables.add(Box::new(TimerDisposable::new(timer)));
    }
}
