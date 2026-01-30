use std::rc::Rc;
use std::time::Duration;

use choreo_models::DancerModel;
use crossbeam_channel::{Receiver, Sender};
use nject::injectable;
use slint::TimerMode;

use crate::behavior::{Behavior, CompositeDisposable, TimerDisposable};
use crate::logging::BehaviorLog;

use super::mapper::{default_role, ensure_default_roles, next_dancer_id};
use super::dancer_settings_view_model::DancerSettingsViewModel;
use super::messages::{AddDancerCommand, DancerSelectionCommand, UpdateSwapSelectionCommand};

#[injectable]
#[inject(
    |receiver: Receiver<AddDancerCommand>,
     selection_sender: Sender<DancerSelectionCommand>,
     swap_selection_sender: Sender<UpdateSwapSelectionCommand>| {
        Self::new(receiver, selection_sender, swap_selection_sender)
    }
)]
pub struct AddDancerBehavior {
    receiver: Receiver<AddDancerCommand>,
    selection_sender: Sender<DancerSelectionCommand>,
    swap_selection_sender: Sender<UpdateSwapSelectionCommand>,
}

impl AddDancerBehavior {
    pub(super) fn new(
        receiver: Receiver<AddDancerCommand>,
        selection_sender: Sender<DancerSelectionCommand>,
        swap_selection_sender: Sender<UpdateSwapSelectionCommand>,
    ) -> Self {
        Self {
            receiver,
            selection_sender,
            swap_selection_sender,
        }
    }

    fn add_dancer(view_model: &mut DancerSettingsViewModel) {
        ensure_default_roles(&mut view_model.roles);

        let next_id = next_dancer_id(&view_model.dancers);
        let role = view_model
            .roles
            .first()
            .cloned()
            .unwrap_or_else(|| Rc::new(default_role("Dame")));
        if !view_model.roles.iter().any(|item| Rc::ptr_eq(item, &role)) {
            view_model.roles.push(role.clone());
        }

        let dancer = Rc::new(DancerModel {
            dancer_id: next_id,
            role: role.clone(),
            name: String::new(),
            shortcut: String::new(),
            color: role.color.clone(),
            icon: None,
        });

        view_model.dancers.push(dancer.clone());
        view_model.selected_dancer = Some(dancer);
    }
}

impl Behavior<DancerSettingsViewModel> for AddDancerBehavior {
    fn activate(
        &self,
        view_model: &mut DancerSettingsViewModel,
        disposables: &mut CompositeDisposable,
    ) {
        BehaviorLog::behavior_activated("AddDancerBehavior", "DancerSettingsViewModel");
        let Some(view_model_handle) = view_model.self_handle().and_then(|handle| handle.upgrade())
        else {
            return;
        };
        let receiver = self.receiver.clone();
        let selection_sender = self.selection_sender.clone();
        let swap_selection_sender = self.swap_selection_sender.clone();
        let timer = slint::Timer::default();
        timer.start(TimerMode::Repeated, Duration::from_millis(16), move || {
            while receiver.try_recv().is_ok() {
                let mut view_model = view_model_handle.borrow_mut();
                Self::add_dancer(&mut view_model);
                let _ = selection_sender.send(DancerSelectionCommand::Refresh);
                let _ = swap_selection_sender.send(UpdateSwapSelectionCommand);
            }
        });
        disposables.add(Box::new(TimerDisposable::new(timer)));
    }
}
