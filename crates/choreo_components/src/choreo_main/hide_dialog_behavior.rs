use crossbeam_channel::Receiver;
use nject::injectable;

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
                view_model.is_dialog_open = false;
                view_model.dialog_content = None;
                true
            }
            Err(_) => false,
        }
    }
}
