use std::rc::Rc;

use choreo_models::DancerModel;
use nject::injectable;

use crate::behavior::{Behavior, CompositeDisposable};
use crate::logging::BehaviorLog;

use super::mapper::normalize_icon_name;
use super::dancer_settings_view_model::DancerSettingsViewModel;

#[injectable]
#[inject(|| Self)]
pub struct SelectedIconBehavior;

impl SelectedIconBehavior {
    pub fn update_selected_icon(view_model: &mut DancerSettingsViewModel) {
        let Some(selected) = view_model.selected_dancer.as_ref() else {
            return;
        };
        let icon_value = view_model
            .selected_icon_option
            .as_ref()
            .map(|option| normalize_icon_name(&option.icon_name));

        let updated = Rc::new(DancerModel {
            icon: icon_value,
            ..(**selected).clone()
        });

        if let Some(index) = view_model
            .dancers
            .iter()
            .position(|dancer| dancer.dancer_id == updated.dancer_id)
        {
            view_model.dancers[index] = updated.clone();
        }
        view_model.selected_dancer = Some(updated);
    }
}

impl Behavior<DancerSettingsViewModel> for SelectedIconBehavior {
    fn activate(
        &self,
        _view_model: &mut DancerSettingsViewModel,
        _disposables: &mut CompositeDisposable,
    ) {
        BehaviorLog::behavior_activated("SelectedIconBehavior", "DancerSettingsViewModel");
    }
}

