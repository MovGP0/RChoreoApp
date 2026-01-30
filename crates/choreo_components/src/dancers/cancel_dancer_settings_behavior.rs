use crate::behavior::{Behavior, CompositeDisposable};
use crate::logging::BehaviorLog;
use nject::injectable;

use super::dancer_settings_view_model::DancerSettingsViewModel;

#[injectable]
#[inject(|| Self)]
pub struct CancelDancerSettingsBehavior;

impl CancelDancerSettingsBehavior {
    pub fn navigate_back() {
        // handled by navigation layer
    }
}

impl Behavior<DancerSettingsViewModel> for CancelDancerSettingsBehavior {
    fn activate(
        &self,
        _view_model: &mut DancerSettingsViewModel,
        _disposables: &mut CompositeDisposable,
    ) {
        BehaviorLog::behavior_activated(
            "CancelDancerSettingsBehavior",
            "DancerSettingsViewModel",
        );
    }
}

