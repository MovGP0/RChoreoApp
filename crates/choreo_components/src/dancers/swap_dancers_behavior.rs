use std::time::Duration;

use crossbeam_channel::Receiver;
use nject::injectable;
use slint::TimerMode;

use crate::behavior::{Behavior, CompositeDisposable, TimerDisposable};
use crate::logging::BehaviorLog;

use super::dancer_settings_view_model::DancerSettingsViewModel;
use super::messages::{ShowDancerDialogCommand, SwapDancersCommand};

#[injectable]
#[inject(
    |show_dialog_sender: crossbeam_channel::Sender<ShowDancerDialogCommand>,
     receiver: Receiver<SwapDancersCommand>| {
        Self::new(show_dialog_sender, receiver)
    }
)]
pub struct SwapDancersBehavior {
    show_dialog_sender: crossbeam_channel::Sender<ShowDancerDialogCommand>,
    receiver: Receiver<SwapDancersCommand>,
}

impl SwapDancersBehavior {
    pub(super) fn new(
        show_dialog_sender: crossbeam_channel::Sender<ShowDancerDialogCommand>,
        receiver: Receiver<SwapDancersCommand>,
    ) -> Self {
        Self {
            show_dialog_sender,
            receiver,
        }
    }

    fn show_swap_dialog(
        show_dialog_sender: &crossbeam_channel::Sender<ShowDancerDialogCommand>,
        view_model: &DancerSettingsViewModel,
    ) {
        let (Some(from), Some(to)) = (
            view_model.swap_from_dancer.as_ref(),
            view_model.swap_to_dancer.as_ref(),
        ) else {
            return;
        };

        if from.dancer_id == to.dancer_id {
            return;
        }

        let _ = show_dialog_sender.send(ShowDancerDialogCommand {
            content_id: Some("swap_dancers".to_string()),
        });
    }
}

impl Behavior<DancerSettingsViewModel> for SwapDancersBehavior {
    fn activate(
        &self,
        view_model: &mut DancerSettingsViewModel,
        disposables: &mut CompositeDisposable,
    ) {
        BehaviorLog::behavior_activated("SwapDancersBehavior", "DancerSettingsViewModel");
        let Some(view_model_handle) = view_model.self_handle().and_then(|handle| handle.upgrade())
        else {
            return;
        };
        let receiver = self.receiver.clone();
        let show_dialog_sender = self.show_dialog_sender.clone();
        let timer = slint::Timer::default();
        timer.start(TimerMode::Repeated, Duration::from_millis(16), move || {
            while receiver.try_recv().is_ok() {
                let view_model = view_model_handle.borrow();
                Self::show_swap_dialog(&show_dialog_sender, &view_model);
            }
        });
        disposables.add(Box::new(TimerDisposable::new(timer)));
    }
}
