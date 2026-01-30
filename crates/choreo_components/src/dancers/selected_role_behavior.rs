use std::rc::Rc;
use std::time::Duration;

use choreo_models::DancerModel;
use crossbeam_channel::Receiver;
use nject::injectable;
use slint::TimerMode;

use crate::behavior::{Behavior, CompositeDisposable, TimerDisposable};
use crate::logging::BehaviorLog;

use super::dancer_settings_view_model::DancerSettingsViewModel;
use super::messages::SelectRoleCommand;

#[injectable]
#[inject(|receiver: Receiver<SelectRoleCommand>| Self { receiver })]
pub struct SelectedRoleBehavior {
    receiver: Receiver<SelectRoleCommand>,
}

impl SelectedRoleBehavior {
    pub(super) fn new(receiver: Receiver<SelectRoleCommand>) -> Self {
        Self { receiver }
    }

    fn update_selected_role(view_model: &mut DancerSettingsViewModel) {
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
        view_model: &mut DancerSettingsViewModel,
        disposables: &mut CompositeDisposable,
    ) {
        BehaviorLog::behavior_activated("SelectedRoleBehavior", "DancerSettingsViewModel");
        let Some(view_model_handle) = view_model.self_handle().and_then(|handle| handle.upgrade())
        else {
            return;
        };
        let receiver = self.receiver.clone();
        let timer = slint::Timer::default();
        timer.start(TimerMode::Repeated, Duration::from_millis(16), move || {
            while let Ok(command) = receiver.try_recv() {
                let mut view_model = view_model_handle.borrow_mut();
                view_model.selected_role = view_model.roles.get(command.index).cloned();
                Self::update_selected_role(&mut view_model);
            }
        });
        disposables.add(Box::new(TimerDisposable::new(timer)));
    }
}
