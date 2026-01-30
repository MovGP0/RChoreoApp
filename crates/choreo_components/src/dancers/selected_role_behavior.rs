use std::rc::Rc;

use choreo_models::DancerModel;
use nject::injectable;

use crate::behavior::{Behavior, CompositeDisposable};
use crate::logging::BehaviorLog;

use super::dancer_settings_view_model::DancerSettingsViewModel;

#[injectable]
#[inject(|| Self)]
pub struct SelectedRoleBehavior;

impl SelectedRoleBehavior {
    pub fn update_selected_role(view_model: &mut DancerSettingsViewModel) {
        let (Some(selected), Some(role)) = (
            view_model.selected_dancer.as_ref(),
            view_model.selected_role.as_ref(),
        ) else {
            return;
        };

        let updated = Rc::new(DancerModel {
            role: role.clone(),
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

impl Behavior<DancerSettingsViewModel> for SelectedRoleBehavior {
    fn activate(
        &self,
        _view_model: &mut DancerSettingsViewModel,
        _disposables: &mut CompositeDisposable,
    ) {
        BehaviorLog::behavior_activated("SelectedRoleBehavior", "DancerSettingsViewModel");
    }
}

