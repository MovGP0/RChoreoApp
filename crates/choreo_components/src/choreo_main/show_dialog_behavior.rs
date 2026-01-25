use crossbeam_channel::Receiver;

use super::messages::ShowDialogCommand;
use super::main_view_model::MainViewModel;

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
                view_model.dialog_content = command.content.clone();
                view_model.is_dialog_open = command.content.is_some();
                true
            }
            Err(_) => false,
        }
    }
}
