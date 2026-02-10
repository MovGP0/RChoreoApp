use std::rc::Rc;
use std::time::Duration;

use choreo_models::DancerModel;
use crossbeam_channel::Receiver;
use nject::injectable;
use slint::TimerMode;

use crate::behavior::{Behavior, CompositeDisposable, TimerDisposable};
use crate::logging::BehaviorLog;

use super::dancer_settings_view_model::DancerSettingsViewModel;
use super::messages::UpdateDancerDetailsCommand;

#[injectable]
#[inject(|receiver: Receiver<UpdateDancerDetailsCommand>| Self { receiver })]
pub struct UpdateDancerDetailsBehavior
{
    receiver: Receiver<UpdateDancerDetailsCommand>,
}

impl UpdateDancerDetailsBehavior
{
    pub(super) fn new(receiver: Receiver<UpdateDancerDetailsCommand>) -> Self
    {
        Self { receiver }
    }

    fn update_selected_dancer(view_model: &mut DancerSettingsViewModel, command: UpdateDancerDetailsCommand)
    {
        let Some(selected) = view_model.selected_dancer.as_ref() else {
            return;
        };

        let updated = match command {
            UpdateDancerDetailsCommand::Name(value) => Rc::new(DancerModel {
                name: value,
                ..(**selected).clone()
            }),
            UpdateDancerDetailsCommand::Shortcut(value) => Rc::new(DancerModel {
                shortcut: value,
                ..(**selected).clone()
            }),
            UpdateDancerDetailsCommand::Color(value) => Rc::new(DancerModel {
                color: value,
                ..(**selected).clone()
            }),
        };

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

impl Behavior<DancerSettingsViewModel> for UpdateDancerDetailsBehavior
{
    fn activate(
        &self,
        view_model: &mut DancerSettingsViewModel,
        disposables: &mut CompositeDisposable,
    )
    {
        BehaviorLog::behavior_activated("UpdateDancerDetailsBehavior", "DancerSettingsViewModel");
        let Some(view_model_handle) = view_model.self_handle().and_then(|handle| handle.upgrade())
        else {
            return;
        };

        let receiver = self.receiver.clone();
        let timer = slint::Timer::default();
        timer.start(TimerMode::Repeated, Duration::from_millis(16), move || {
            while let Ok(command) = receiver.try_recv() {
                let mut view_model = view_model_handle.borrow_mut();
                Self::update_selected_dancer(&mut view_model, command);
            }
        });
        disposables.add(Box::new(TimerDisposable::new(timer)));
    }
}
