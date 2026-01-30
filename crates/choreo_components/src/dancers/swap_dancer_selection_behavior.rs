use crate::behavior::{Behavior, CompositeDisposable};
use crate::logging::BehaviorLog;
use nject::injectable;

use super::dancer_settings_view_model::DancerSettingsViewModel;

#[injectable]
#[inject(|| Self)]
pub struct SwapDancerSelectionBehavior;

impl SwapDancerSelectionBehavior {
    pub fn ensure_swap_selections(view_model: &mut DancerSettingsViewModel) {
        if view_model.dancers.is_empty() {
            view_model.swap_from_dancer = None;
            view_model.swap_to_dancer = None;
            Self::update_can_swap(view_model);
            return;
        }

        if view_model
            .swap_from_dancer
            .as_ref()
            .map(|dancer| !view_model.dancers.iter().any(|item| item.dancer_id == dancer.dancer_id))
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
            .map(|dancer| !view_model.dancers.iter().any(|item| item.dancer_id == dancer.dancer_id))
            .unwrap_or(true)
            || view_model.swap_to_dancer.as_ref().map(|dancer| {
                view_model
                    .swap_from_dancer
                    .as_ref()
                    .map(|from| from.dancer_id == dancer.dancer_id)
                    .unwrap_or(false)
            }).unwrap_or(false)
        {
            view_model.swap_to_dancer = view_model.dancers.iter().find(|dancer| {
                view_model
                    .swap_from_dancer
                    .as_ref()
                    .map(|from| from.dancer_id != dancer.dancer_id)
                    .unwrap_or(true)
            }).cloned();
        }

        Self::update_can_swap(view_model);
    }

    pub fn update_can_swap(view_model: &mut DancerSettingsViewModel) {
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
    fn initialize(
        &self,
        _view_model: &mut DancerSettingsViewModel,
        _disposables: &mut CompositeDisposable,
    ) {
        BehaviorLog::behavior_activated(
            "SwapDancerSelectionBehavior",
            "DancerSettingsViewModel",
        );
    }
}

