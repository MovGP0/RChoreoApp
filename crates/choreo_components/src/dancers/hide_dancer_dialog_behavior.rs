use std::time::Duration;

use crossbeam_channel::Receiver;
use nject::injectable;
use slint::TimerMode;

use crate::behavior::{Behavior, CompositeDisposable, TimerDisposable};
use crate::logging::BehaviorLog;

use super::dancer_settings_view_model::DancerSettingsViewModel;
use super::messages::CloseDancerDialogCommand;

#[injectable]
#[inject(|receiver: Receiver<CloseDancerDialogCommand>| Self { receiver })]
pub struct HideDancerDialogBehavior {
    receiver: Receiver<CloseDancerDialogCommand>,
}

impl HideDancerDialogBehavior {
    pub(super) fn new(receiver: Receiver<CloseDancerDialogCommand>) -> Self {
        Self { receiver }
    }

    fn hide_dialog(view_model: &mut DancerSettingsViewModel) {
        view_model.is_dialog_open = false;
        view_model.dialog_content = None;
    }
}

impl Behavior<DancerSettingsViewModel> for HideDancerDialogBehavior {
    fn activate(
        &self,
        view_model: &mut DancerSettingsViewModel,
        disposables: &mut CompositeDisposable,
    ) {
        BehaviorLog::behavior_activated("HideDancerDialogBehavior", "DancerSettingsViewModel");
        let Some(view_model_handle) = view_model.self_handle().and_then(|handle| handle.upgrade())
        else {
            return;
        };
        let receiver = self.receiver.clone();
        let timer = slint::Timer::default();
        timer.start(TimerMode::Repeated, Duration::from_millis(16), move || {
            while receiver.try_recv().is_ok() {
                let mut view_model = view_model_handle.borrow_mut();
                Self::hide_dialog(&mut view_model);
            }
        });
        disposables.add(Box::new(TimerDisposable::new(timer)));
    }
}
