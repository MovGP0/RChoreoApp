use crossbeam_channel::Sender;
use nject::injectable;

use crate::behavior::{Behavior, CompositeDisposable};
use crate::logging::BehaviorLog;

use super::main_view_model::MainViewModel;
use super::messages::OpenSvgFileCommand;

#[injectable]
#[inject(|sender: Sender<OpenSvgFileCommand>| Self { sender })]
pub struct OpenImageBehavior {
    sender: Sender<OpenSvgFileCommand>,
}

impl OpenImageBehavior {
    pub fn new(sender: Sender<OpenSvgFileCommand>) -> Self {
        Self { sender }
    }

    pub fn open_svg(&self, path: String) {
        let _ = self.sender.send(OpenSvgFileCommand { file_path: path });
    }
}

impl Behavior<MainViewModel> for OpenImageBehavior {
    fn activate(&self, _view_model: &mut MainViewModel, _disposables: &mut CompositeDisposable) {
        BehaviorLog::behavior_activated("OpenImageBehavior", "MainViewModel");
    }
}
