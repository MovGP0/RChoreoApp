use std::time::Duration;

use crossbeam_channel::Receiver;
use nject::injectable;
use slint::TimerMode;

use crate::behavior::{Behavior, CompositeDisposable, TimerDisposable};
use crate::logging::BehaviorLog;

use super::dancer_settings_view_model::DancerSettingsViewModel;
use super::messages::ShowDancerDialogCommand;

#[injectable]
#[inject(|receiver: Receiver<ShowDancerDialogCommand>| Self { receiver })]
pub struct ShowDancerDialogBehavior {
    receiver: Receiver<ShowDancerDialogCommand>,
}

impl ShowDancerDialogBehavior {
    pub(super) fn new(receiver: Receiver<ShowDancerDialogCommand>) -> Self {
        Self { receiver }
    }
}

impl Behavior<DancerSettingsViewModel> for ShowDancerDialogBehavior {
    fn activate(
        &self,
        view_model: &mut DancerSettingsViewModel,
        disposables: &mut CompositeDisposable,
    ) {
        BehaviorLog::behavior_activated("ShowDancerDialogBehavior", "DancerSettingsViewModel");
        let Some(view_model_handle) = view_model.self_handle().and_then(|handle| handle.upgrade())
        else {
            return;
        };
        let receiver = self.receiver.clone();
        let timer = slint::Timer::default();
        timer.start(TimerMode::Repeated, Duration::from_millis(16), move || {
            while let Ok(command) = receiver.try_recv() {
                let mut view_model = view_model_handle.borrow_mut();
                view_model.dialog_content = command.content_id.clone();
                view_model.is_dialog_open = command.content_id.is_some();
            }
        });
        disposables.add(Box::new(TimerDisposable::new(timer)));
    }
}
