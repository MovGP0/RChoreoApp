use std::time::Duration;

use crossbeam_channel::Receiver;
use nject::injectable;
use slint::TimerMode;

use crate::behavior::{Behavior, CompositeDisposable};
use crate::behavior::TimerDisposable;
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
}

impl Behavior<MainViewModel> for ShowDialogBehavior {
    fn activate(&self, view_model: &mut MainViewModel, disposables: &mut CompositeDisposable) {
        BehaviorLog::behavior_activated("ShowDialogBehavior", "MainViewModel");
        let Some(view_model_handle) = view_model.self_handle().and_then(|handle| handle.upgrade())
        else {
            return;
        };
        let receiver = self.receiver.clone();
        let timer = slint::Timer::default();
        timer.start(TimerMode::Repeated, Duration::from_millis(16), move || {
            while let Ok(command) = receiver.try_recv() {
                view_model_handle.borrow_mut().show_dialog(command.content);
            }
        });
        disposables.add(Box::new(TimerDisposable::new(timer)));
    }
}
