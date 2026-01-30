use crossbeam_channel::Receiver;
use nject::injectable;

use crate::behavior::{Behavior, CompositeDisposable};
use crate::logging::BehaviorLog;

use super::messages::CloseDialogCommand;
use super::main_view_model::MainViewModel;

#[injectable]
#[inject(|receiver: Receiver<CloseDialogCommand>| Self { receiver })]
pub struct HideDialogBehavior {
    receiver: Receiver<CloseDialogCommand>,
}

impl HideDialogBehavior {
    pub fn new(receiver: Receiver<CloseDialogCommand>) -> Self {
        Self { receiver }
    }

    pub fn try_handle(&self, view_model: &mut MainViewModel) -> bool {
        match self.receiver.try_recv() {
            Ok(_) => {
                view_model.hide_dialog();
                true
            }
            Err(_) => false,
        }
    }
}

impl Behavior<MainViewModel> for HideDialogBehavior {
    fn activate(&self, _view_model: &mut MainViewModel, _disposables: &mut CompositeDisposable) {
        BehaviorLog::behavior_activated("HideDialogBehavior", "MainViewModel");
    }
}
