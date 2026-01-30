use crate::behavior::{Behavior, CompositeDisposable};
use crate::logging::BehaviorLog;
use nject::injectable;

use super::mapper::is_icon_match;
use super::dancer_settings_view_model::DancerSettingsViewModel;

#[injectable]
#[inject(|| Self)]
pub struct SelectedDancerStateBehavior;

impl SelectedDancerStateBehavior {
    pub fn update_selected_dancer(view_model: &mut DancerSettingsViewModel) {
        let dancer = view_model.selected_dancer.clone();
        view_model.has_selected_dancer = dancer.is_some();
        view_model.can_delete_dancer = dancer.is_some();
        view_model.selected_icon_option = dancer
            .as_ref()
            .and_then(|value| {
                view_model
                    .icon_options
                    .iter()
                    .find(|option| is_icon_match(option, value.icon.as_deref()))
                    .cloned()
            });
        view_model.selected_role = dancer.as_ref().map(|value| value.role.clone());
    }
}

impl Behavior<DancerSettingsViewModel> for SelectedDancerStateBehavior {
    fn activate(
        &self,
        _view_model: &mut DancerSettingsViewModel,
        _disposables: &mut CompositeDisposable,
    ) {
        BehaviorLog::behavior_activated(
            "SelectedDancerStateBehavior",
            "DancerSettingsViewModel",
        );
    }
}

