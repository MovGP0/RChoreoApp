use crossbeam_channel::Receiver;
use nject::injectable;

use crate::behavior::{Behavior, CompositeDisposable};
use crate::logging::BehaviorLog;

use super::messages::ShowDialogCommand;
use super::main_view_model::MainViewModel;

#[injectable]
#[inject(|receiver: Receiver<ShowDialogCommand>| Self { receiver })]
pub struct ShowDialogBehavior {
    receiver: Receiver<ShowDialogCommand>,
}

impl ShowDialogBehavior {
    pub fn new(receiver: Receiver<ShowDialogCommand>) -> Self {
        Self { receiver }
    }

    pub fn try_handle(&self, view_model: &mut MainViewModel) -> bool {
        match self.receiver.try_recv() {
            Ok(command) => {
                view_model.show_dialog(command.content);
                true
            }
            Err(_) => false,
        }
    }
}

impl Behavior<MainViewModel> for ShowDialogBehavior {
    fn activate(&self, _view_model: &mut MainViewModel, _disposables: &mut CompositeDisposable) {
        BehaviorLog::behavior_activated("ShowDialogBehavior", "MainViewModel");
    }
}
