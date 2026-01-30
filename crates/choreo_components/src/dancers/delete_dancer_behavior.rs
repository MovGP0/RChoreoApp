use crate::behavior::{Behavior, CompositeDisposable};
use crate::logging::BehaviorLog;
use nject::injectable;

use super::dancer_settings_view_model::DancerSettingsViewModel;

#[injectable]
#[inject(|| Self)]
pub struct DeleteDancerBehavior;

impl DeleteDancerBehavior {
    pub fn delete_dancer(view_model: &mut DancerSettingsViewModel) {
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
    fn initialize(
        &self,
        _view_model: &mut DancerSettingsViewModel,
        _disposables: &mut CompositeDisposable,
    ) {
        BehaviorLog::behavior_activated("DeleteDancerBehavior", "DancerSettingsViewModel");
    }
}

