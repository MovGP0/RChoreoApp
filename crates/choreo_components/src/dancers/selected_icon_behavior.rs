use std::rc::Rc;
use std::time::Duration;

use choreo_models::DancerModel;
use crossbeam_channel::Receiver;
use nject::injectable;
use slint::TimerMode;

use crate::behavior::{Behavior, CompositeDisposable, TimerDisposable};
use crate::logging::BehaviorLog;

use super::mapper::normalize_icon_name;
use super::dancer_settings_view_model::DancerSettingsViewModel;
use super::messages::UpdateDancerIconCommand;

#[injectable]
#[inject(|receiver: Receiver<UpdateDancerIconCommand>| Self { receiver })]
pub struct SelectedIconBehavior {
    receiver: Receiver<UpdateDancerIconCommand>,
}

impl SelectedIconBehavior {
    pub(super) fn new(receiver: Receiver<UpdateDancerIconCommand>) -> Self {
        Self { receiver }
    }

    fn update_selected_icon(view_model: &mut DancerSettingsViewModel) {
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
        view_model: &mut DancerSettingsViewModel,
        disposables: &mut CompositeDisposable,
    ) {
        BehaviorLog::behavior_activated("SelectedIconBehavior", "DancerSettingsViewModel");
        let Some(view_model_handle) = view_model.self_handle().and_then(|handle| handle.upgrade())
        else {
            return;
        };
        let receiver = self.receiver.clone();
        let timer = slint::Timer::default();
        timer.start(TimerMode::Repeated, Duration::from_millis(16), move || {
            while let Ok(command) = receiver.try_recv() {
                let value = command.value;
                let mut view_model = view_model_handle.borrow_mut();
                view_model.selected_icon_option = if value.trim().is_empty() {
                    None
                } else {
                    view_model
                        .icon_options
                        .iter()
                        .find(|option| {
                            option.icon_name.eq_ignore_ascii_case(&value)
                                || option.key.eq_ignore_ascii_case(&value)
                        })
                        .cloned()
                };
                Self::update_selected_icon(&mut view_model);
            }
        });
        disposables.add(Box::new(TimerDisposable::new(timer)));
    }
}
