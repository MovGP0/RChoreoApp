use std::time::Duration;

use crossbeam_channel::{Receiver, Sender};
use nject::injectable;
use slint::TimerMode;

use crate::behavior::{Behavior, CompositeDisposable, TimerDisposable};
use crate::logging::BehaviorLog;

use super::dancer_settings_view_model::DancerSettingsViewModel;
use super::messages::{DancerSelectionCommand, SwapDancersCommand};

#[injectable]
#[inject(
    |selection_sender: Sender<DancerSelectionCommand>,
     receiver: Receiver<SwapDancersCommand>| {
        Self::new(selection_sender, receiver)
    }
)]
pub struct SwapDancersBehavior {
    selection_sender: Sender<DancerSelectionCommand>,
    receiver: Receiver<SwapDancersCommand>,
}

impl SwapDancersBehavior {
    pub(super) fn new(
        selection_sender: Sender<DancerSelectionCommand>,
        receiver: Receiver<SwapDancersCommand>,
    ) -> Self {
        Self {
            selection_sender,
            receiver,
        }
    }

    fn swap_dancers(view_model: &mut DancerSettingsViewModel) -> bool {
        let (Some(from), Some(to)) = (
            view_model.swap_from_dancer.as_ref(),
            view_model.swap_to_dancer.as_ref(),
        ) else {
            return false;
        };

        if from.dancer_id == to.dancer_id {
            return false;
        }

        let Some(from_index) = view_model
            .dancers
            .iter()
            .position(|dancer| dancer.dancer_id == from.dancer_id)
        else {
            return false;
        };
        let Some(to_index) = view_model
            .dancers
            .iter()
            .position(|dancer| dancer.dancer_id == to.dancer_id)
        else {
            return false;
        };

        if from_index == to_index {
            return false;
        }

        let from_role = view_model.dancers[from_index].role.clone();
        let from_name = view_model.dancers[from_index].name.clone();
        let from_shortcut = view_model.dancers[from_index].shortcut.clone();
        let from_color = view_model.dancers[from_index].color.clone();
        let from_icon = view_model.dancers[from_index].icon.clone();
        let to_role = view_model.dancers[to_index].role.clone();
        let to_name = view_model.dancers[to_index].name.clone();
        let to_shortcut = view_model.dancers[to_index].shortcut.clone();
        let to_color = view_model.dancers[to_index].color.clone();
        let to_icon = view_model.dancers[to_index].icon.clone();

        {
            let from_mut = std::rc::Rc::make_mut(&mut view_model.dancers[from_index]);
            from_mut.role = to_role;
            from_mut.name = to_name;
            from_mut.shortcut = to_shortcut;
            from_mut.color = to_color;
            from_mut.icon = to_icon;
        }

        {
            let to_mut = std::rc::Rc::make_mut(&mut view_model.dancers[to_index]);
            to_mut.role = from_role;
            to_mut.name = from_name;
            to_mut.shortcut = from_shortcut;
            to_mut.color = from_color;
            to_mut.icon = from_icon;
        }

        if let Some(selected) = view_model.selected_dancer.as_ref() {
            if selected.dancer_id == from.dancer_id {
                view_model.selected_dancer = view_model.dancers.get(from_index).cloned();
            } else if selected.dancer_id == to.dancer_id {
                view_model.selected_dancer = view_model.dancers.get(to_index).cloned();
            }
        }

        view_model.swap_from_dancer = view_model.dancers.get(from_index).cloned();
        view_model.swap_to_dancer = view_model.dancers.get(to_index).cloned();

        true
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
        let selection_sender = self.selection_sender.clone();
        let timer = slint::Timer::default();
        timer.start(TimerMode::Repeated, Duration::from_millis(16), move || {
            while receiver.try_recv().is_ok() {
                let mut view_model = view_model_handle.borrow_mut();
                if Self::swap_dancers(&mut view_model) {
                    let _ = selection_sender.send(DancerSelectionCommand::Refresh);
                }
            }
        });
        disposables.add(Box::new(TimerDisposable::new(timer)));
    }
}
