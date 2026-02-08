use std::time::Duration;

use crossbeam_channel::Receiver;
use nject::injectable;
use slint::TimerMode;

use crate::behavior::{Behavior, CompositeDisposable, TimerDisposable};
use crate::logging::BehaviorLog;

use super::dancer_settings_view_model::DancerSettingsViewModel;
use super::messages::UpdateSwapSelectionCommand;

#[injectable]
#[inject(|receiver: Receiver<UpdateSwapSelectionCommand>| Self { receiver })]
pub struct SwapDancerSelectionBehavior {
    receiver: Receiver<UpdateSwapSelectionCommand>,
}

impl SwapDancerSelectionBehavior {
    pub(super) fn new(receiver: Receiver<UpdateSwapSelectionCommand>) -> Self {
        Self { receiver }
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
        let receiver = self.receiver.clone();
        let timer = slint::Timer::default();
        timer.start(TimerMode::Repeated, Duration::from_millis(16), move || {
            while receiver.try_recv().is_ok() {
                let mut view_model = view_model_handle.borrow_mut();
                Self::ensure_swap_selections(&mut view_model);
            }
        });
        disposables.add(Box::new(TimerDisposable::new(timer)));
    }
}
