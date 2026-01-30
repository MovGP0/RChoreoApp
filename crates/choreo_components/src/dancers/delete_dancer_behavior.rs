use std::time::Duration;

use crossbeam_channel::{Receiver, Sender};
use nject::injectable;
use slint::TimerMode;

use crate::behavior::{Behavior, CompositeDisposable, TimerDisposable};
use crate::logging::BehaviorLog;

use super::dancer_settings_view_model::DancerSettingsViewModel;
use super::messages::{DancerSelectionCommand, DeleteDancerCommand, UpdateSwapSelectionCommand};

#[injectable]
#[inject(
    |receiver: Receiver<DeleteDancerCommand>,
     selection_sender: Sender<DancerSelectionCommand>,
     swap_selection_sender: Sender<UpdateSwapSelectionCommand>| {
        Self::new(receiver, selection_sender, swap_selection_sender)
    }
)]
pub struct DeleteDancerBehavior {
    receiver: Receiver<DeleteDancerCommand>,
    selection_sender: Sender<DancerSelectionCommand>,
    swap_selection_sender: Sender<UpdateSwapSelectionCommand>,
}

impl DeleteDancerBehavior {
    pub(super) fn new(
        receiver: Receiver<DeleteDancerCommand>,
        selection_sender: Sender<DancerSelectionCommand>,
        swap_selection_sender: Sender<UpdateSwapSelectionCommand>,
    ) -> Self {
        Self {
            receiver,
            selection_sender,
            swap_selection_sender,
        }
    }

    fn delete_dancer(view_model: &mut DancerSettingsViewModel) {
        let Some(selected) = view_model.selected_dancer.as_ref() else {
            return;
        };
        let selected_id = selected.dancer_id;

        let index = view_model
            .dancers
            .iter()
            .position(|dancer| dancer.dancer_id == selected_id);
        let Some(index) = index else {
            return;
        };

        view_model.dancers.remove(index);
        view_model.selected_dancer = if view_model.dancers.is_empty() {
            None
        } else {
            Some(view_model.dancers[std::cmp::min(index, view_model.dancers.len() - 1)].clone())
        };
    }
}

impl Behavior<DancerSettingsViewModel> for DeleteDancerBehavior {
    fn activate(
        &self,
        view_model: &mut DancerSettingsViewModel,
        disposables: &mut CompositeDisposable,
    ) {
        BehaviorLog::behavior_activated("DeleteDancerBehavior", "DancerSettingsViewModel");
        let Some(view_model_handle) = view_model.self_handle().and_then(|handle| handle.upgrade())
        else {
            return;
        };
        let receiver = self.receiver.clone();
        let selection_sender = self.selection_sender.clone();
        let swap_selection_sender = self.swap_selection_sender.clone();
        let timer = slint::Timer::default();
        timer.start(TimerMode::Repeated, Duration::from_millis(16), move || {
            while receiver.try_recv().is_ok() {
                {
                    let mut view_model = view_model_handle.borrow_mut();
                    Self::delete_dancer(&mut view_model);
                }
                let _ = selection_sender.send(DancerSelectionCommand::Refresh);
                let _ = swap_selection_sender.send(UpdateSwapSelectionCommand);
            }
        });
        disposables.add(Box::new(TimerDisposable::new(timer)));
    }
}
