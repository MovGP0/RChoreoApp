use std::time::Duration;

use crossbeam_channel::Receiver;
use nject::injectable;
use slint::TimerMode;

use crate::behavior::{Behavior, CompositeDisposable, TimerDisposable};
use crate::logging::BehaviorLog;

use super::dancer_settings_view_model::DancerSettingsViewModel;
use super::messages::{
    UpdateSwapFromCommand,
    UpdateSwapSelectionCommand,
    UpdateSwapToCommand,
};

#[injectable]
#[inject(
    |refresh_receiver: Receiver<UpdateSwapSelectionCommand>,
     from_receiver: Receiver<UpdateSwapFromCommand>,
     to_receiver: Receiver<UpdateSwapToCommand>| {
        Self {
            refresh_receiver,
            from_receiver,
            to_receiver
        }
    }
)]
pub struct SwapDancerSelectionBehavior {
    refresh_receiver: Receiver<UpdateSwapSelectionCommand>,
    from_receiver: Receiver<UpdateSwapFromCommand>,
    to_receiver: Receiver<UpdateSwapToCommand>,
}

impl SwapDancerSelectionBehavior {
    pub(super) fn new(
        refresh_receiver: Receiver<UpdateSwapSelectionCommand>,
        from_receiver: Receiver<UpdateSwapFromCommand>,
        to_receiver: Receiver<UpdateSwapToCommand>,
    ) -> Self {
        Self {
            refresh_receiver,
            from_receiver,
            to_receiver,
        }
    }

    fn ensure_swap_selections(view_model: &mut DancerSettingsViewModel) {
        if view_model.dancers.is_empty() {
            view_model.swap_from_dancer = None;
            view_model.swap_to_dancer = None;
            Self::update_can_swap(view_model);
            return;
        }

        if view_model
            .swap_from_dancer
            .as_ref()
            .map(|dancer| {
                !view_model
                    .dancers
                    .iter()
                    .any(|item| item.dancer_id == dancer.dancer_id)
            })
            .unwrap_or(true)
        {
            view_model.swap_from_dancer = view_model.dancers.first().cloned();
        }

        if view_model.dancers.len() < 2 {
            view_model.swap_to_dancer = None;
            Self::update_can_swap(view_model);
            return;
        }

        if view_model
            .swap_to_dancer
            .as_ref()
            .map(|dancer| {
                !view_model
                    .dancers
                    .iter()
                    .any(|item| item.dancer_id == dancer.dancer_id)
            })
            .unwrap_or(true)
            || view_model
                .swap_to_dancer
                .as_ref()
                .map(|dancer| {
                    view_model
                        .swap_from_dancer
                        .as_ref()
                        .map(|from| from.dancer_id == dancer.dancer_id)
                        .unwrap_or(false)
                })
                .unwrap_or(false)
        {
            view_model.swap_to_dancer = view_model
                .dancers
                .iter()
                .find(|dancer| {
                    view_model
                        .swap_from_dancer
                        .as_ref()
                        .map(|from| from.dancer_id != dancer.dancer_id)
                        .unwrap_or(true)
                })
                .cloned();
        }

        Self::update_can_swap(view_model);
    }

    fn update_swap_from(view_model: &mut DancerSettingsViewModel, index: usize) {
        view_model.swap_from_dancer = view_model.dancers.get(index).cloned();
        Self::update_can_swap(view_model);
    }

    fn update_swap_to(view_model: &mut DancerSettingsViewModel, index: usize) {
        view_model.swap_to_dancer = view_model.dancers.get(index).cloned();
        Self::update_can_swap(view_model);
    }

    fn update_can_swap(view_model: &mut DancerSettingsViewModel) {
        view_model.can_swap_dancers = view_model.swap_from_dancer.is_some()
            && view_model.swap_to_dancer.is_some()
            && view_model
                .swap_from_dancer
                .as_ref()
                .zip(view_model.swap_to_dancer.as_ref())
                .map(|(from, to)| from.dancer_id != to.dancer_id)
                .unwrap_or(false);
    }
}

impl Behavior<DancerSettingsViewModel> for SwapDancerSelectionBehavior {
    fn activate(
        &self,
        view_model: &mut DancerSettingsViewModel,
        disposables: &mut CompositeDisposable,
    ) {
        BehaviorLog::behavior_activated("SwapDancerSelectionBehavior", "DancerSettingsViewModel");
        let Some(view_model_handle) = view_model.self_handle().and_then(|handle| handle.upgrade())
        else {
            return;
        };
        let refresh_receiver = self.refresh_receiver.clone();
        let from_receiver = self.from_receiver.clone();
        let to_receiver = self.to_receiver.clone();
        let timer = slint::Timer::default();
        timer.start(TimerMode::Repeated, Duration::from_millis(16), move || {
            while refresh_receiver.try_recv().is_ok() {
                let mut view_model = view_model_handle.borrow_mut();
                Self::ensure_swap_selections(&mut view_model);
            }
            while let Ok(command) = from_receiver.try_recv() {
                let mut view_model = view_model_handle.borrow_mut();
                Self::update_swap_from(&mut view_model, command.index);
            }
            while let Ok(command) = to_receiver.try_recv() {
                let mut view_model = view_model_handle.borrow_mut();
                Self::update_swap_to(&mut view_model, command.index);
            }
        });
        disposables.add(Box::new(TimerDisposable::new(timer)));
    }
}
