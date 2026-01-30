use std::time::Duration;

use crossbeam_channel::Receiver;
use nject::injectable;
use slint::TimerMode;

use crate::behavior::{Behavior, CompositeDisposable, TimerDisposable};
use crate::logging::BehaviorLog;

use super::mapper::is_icon_match;
use super::dancer_settings_view_model::DancerSettingsViewModel;
use super::messages::DancerSelectionCommand;

#[injectable]
#[inject(|receiver: Receiver<DancerSelectionCommand>| Self { receiver })]
pub struct SelectedDancerStateBehavior {
    receiver: Receiver<DancerSelectionCommand>,
}

impl SelectedDancerStateBehavior {
    pub(super) fn new(receiver: Receiver<DancerSelectionCommand>) -> Self {
        Self { receiver }
    }

    fn update_selected_dancer(view_model: &mut DancerSettingsViewModel) {
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
        view_model: &mut DancerSettingsViewModel,
        disposables: &mut CompositeDisposable,
    ) {
        BehaviorLog::behavior_activated(
            "SelectedDancerStateBehavior",
            "DancerSettingsViewModel",
        );
        let Some(view_model_handle) = view_model.self_handle().and_then(|handle| handle.upgrade())
        else {
            return;
        };
        let receiver = self.receiver.clone();
        let timer = slint::Timer::default();
        timer.start(TimerMode::Repeated, Duration::from_millis(16), move || {
            while let Ok(command) = receiver.try_recv() {
                let mut view_model = view_model_handle.borrow_mut();
                if let DancerSelectionCommand::Select(index) = command {
                    view_model.selected_dancer = view_model.dancers.get(index).cloned();
                }
                Self::update_selected_dancer(&mut view_model);
            }
        });
        disposables.add(Box::new(TimerDisposable::new(timer)));
    }
}
