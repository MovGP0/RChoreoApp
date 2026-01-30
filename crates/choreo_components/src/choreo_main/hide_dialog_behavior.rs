use std::time::Duration;

use crossbeam_channel::Receiver;
use nject::injectable;
use slint::TimerMode;

use crate::behavior::{Behavior, CompositeDisposable};
use crate::behavior::TimerDisposable;
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
}

impl Behavior<MainViewModel> for HideDialogBehavior {
    fn activate(&self, view_model: &mut MainViewModel, disposables: &mut CompositeDisposable) {
        BehaviorLog::behavior_activated("HideDialogBehavior", "MainViewModel");
        let Some(view_model_handle) = view_model.self_handle().and_then(|handle| handle.upgrade())
        else {
            return;
        };
        let receiver = self.receiver.clone();
        let timer = slint::Timer::default();
        timer.start(TimerMode::Repeated, Duration::from_millis(16), move || {
            while receiver.try_recv().is_ok() {
                view_model_handle.borrow_mut().hide_dialog();
            }
        });
        disposables.add(Box::new(TimerDisposable::new(timer)));
    }
}
