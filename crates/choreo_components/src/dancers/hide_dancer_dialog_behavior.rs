use crossbeam_channel::Receiver;
use nject::injectable;

use crate::behavior::{Behavior, CompositeDisposable};
use crate::logging::BehaviorLog;

use super::dancer_settings_view_model::DancerSettingsViewModel;
use super::messages::CloseDancerDialogCommand;

#[injectable]
#[inject(|receiver: Receiver<CloseDancerDialogCommand>| Self { receiver })]
pub struct HideDancerDialogBehavior {
    receiver: Receiver<CloseDancerDialogCommand>,
}

impl HideDancerDialogBehavior {
    pub fn new(receiver: Receiver<CloseDancerDialogCommand>) -> Self {
        Self { receiver }
    }

    pub fn hide_dialog(view_model: &mut DancerSettingsViewModel) {
        view_model.is_dialog_open = false;
        view_model.dialog_content = None;
    }

    pub fn try_handle(&self, view_model: &mut DancerSettingsViewModel) -> bool {
        match self.receiver.try_recv() {
            Ok(_) => {
                Self::hide_dialog(view_model);
                true
            }
            Err(_) => false,
        }
    }
}

impl Behavior<DancerSettingsViewModel> for HideDancerDialogBehavior {
    fn initialize(
        &self,
        _view_model: &mut DancerSettingsViewModel,
        _disposables: &mut CompositeDisposable,
    ) {
        BehaviorLog::behavior_activated("HideDancerDialogBehavior", "DancerSettingsViewModel");
    }
}

