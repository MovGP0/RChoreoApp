use std::time::Duration;

use crossbeam_channel::Receiver;
use nject::injectable;
use slint::TimerMode;

use crate::behavior::{Behavior, CompositeDisposable, TimerDisposable};
use crate::logging::BehaviorLog;

use super::dancer_settings_view_model::DancerSettingsViewModel;
use super::messages::CancelDancerSettingsCommand;

#[injectable]
#[inject(|receiver: Receiver<CancelDancerSettingsCommand>| Self { receiver })]
pub struct CancelDancerSettingsBehavior {
    receiver: Receiver<CancelDancerSettingsCommand>,
}

impl CancelDancerSettingsBehavior {
    pub(super) fn new(receiver: Receiver<CancelDancerSettingsCommand>) -> Self {
        Self { receiver }
    }

    fn navigate_back() {
        // handled by navigation layer
    }
}

impl Behavior<DancerSettingsViewModel> for CancelDancerSettingsBehavior {
    fn activate(
        &self,
        view_model: &mut DancerSettingsViewModel,
        disposables: &mut CompositeDisposable,
    ) {
        BehaviorLog::behavior_activated(
            "CancelDancerSettingsBehavior",
            "DancerSettingsViewModel",
        );
        let Some(_view_model_handle) = view_model.self_handle().and_then(|handle| handle.upgrade())
        else {
            return;
        };
        let receiver = self.receiver.clone();
        let timer = slint::Timer::default();
        timer.start(TimerMode::Repeated, Duration::from_millis(16), move || {
            while receiver.try_recv().is_ok() {
                let _ = _view_model_handle;
                Self::navigate_back();
            }
        });
        disposables.add(Box::new(TimerDisposable::new(timer)));
    }
}
