use crossbeam_channel::Receiver;

use crate::behavior::{Behavior, CompositeDisposable};
use crate::logging::BehaviorLog;

use super::dancer_settings_view_model::DancerSettingsViewModel;
use super::messages::ShowDancerDialogCommand;

pub struct ShowDancerDialogBehavior {
    receiver: Receiver<ShowDancerDialogCommand>,
}

impl ShowDancerDialogBehavior {
    pub fn new(receiver: Receiver<ShowDancerDialogCommand>) -> Self {
        Self { receiver }
    }

    pub fn try_handle(&self, view_model: &mut DancerSettingsViewModel) -> bool {
        match self.receiver.try_recv() {
            Ok(command) => {
                view_model.dialog_content = command.content_id.clone();
                view_model.is_dialog_open = command.content_id.is_some();
                true
            }
            Err(_) => false,
        }
    }
}

impl Behavior<DancerSettingsViewModel> for ShowDancerDialogBehavior {
    fn activate(
        &self,
        _view_model: &mut DancerSettingsViewModel,
        _disposables: &mut CompositeDisposable,
    ) {
        BehaviorLog::behavior_activated("ShowDancerDialogBehavior", "DancerSettingsViewModel");
    }
}

