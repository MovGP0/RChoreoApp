use std::collections::HashMap;
use std::rc::Rc;
use std::time::Duration;

use choreo_models::{
    DancerModel,
    RoleModel
};
use crossbeam_channel::{
    Receiver,
    Sender
};
use nject::injectable;
use slint::TimerMode;

use crate::behavior::{
    Behavior,
    CompositeDisposable,
    TimerDisposable
};
use crate::global::GlobalStateActor;
use crate::logging::BehaviorLog;

use super::dancer_settings_view_model::DancerSettingsViewModel;
use super::mapper::{default_role, ensure_default_roles};
use super::messages::{
    DancerSelectionCommand,
    ReloadDancerSettingsCommand,
    UpdateSwapSelectionCommand
};

#[injectable]
#[inject(
    |global_state: Rc<GlobalStateActor>,
     reload_receiver: Receiver<ReloadDancerSettingsCommand>,
     selection_sender: Sender<DancerSelectionCommand>,
     swap_selection_sender: Sender<UpdateSwapSelectionCommand>| {
        Self::new(
            global_state,
            reload_receiver,
            selection_sender,
            swap_selection_sender
        )
    }
)]
pub struct LoadDancerSettingsBehavior {
    global_state: Rc<GlobalStateActor>,
    reload_receiver: Receiver<ReloadDancerSettingsCommand>,
    selection_sender: Sender<DancerSelectionCommand>,
    swap_selection_sender: Sender<UpdateSwapSelectionCommand>,
}

impl LoadDancerSettingsBehavior {
    pub(super) fn new(
        global_state: Rc<GlobalStateActor>,
        reload_receiver: Receiver<ReloadDancerSettingsCommand>,
        selection_sender: Sender<DancerSelectionCommand>,
        swap_selection_sender: Sender<UpdateSwapSelectionCommand>,
    ) -> Self {
        Self {
            global_state,
            reload_receiver,
            selection_sender,
            swap_selection_sender,
        }
    }

    pub(super) fn load(&self, view_model: &mut DancerSettingsViewModel) {
        view_model.roles.clear();
        view_model.dancers.clear();

        let Some(snapshot) = self.global_state.try_with_state(|global_state| {
            (
                global_state.choreography.roles.clone(),
                global_state.choreography.dancers.clone(),
            )
        }) else {
            return;
        };

        let mut role_map: HashMap<usize, Rc<RoleModel>> = HashMap::new();
        for role in &snapshot.0 {
            let copy = Rc::new(RoleModel {
                name: role.name.clone(),
                color: role.color.clone(),
                z_index: role.z_index,
            });
            role_map.insert(Rc::as_ptr(role) as usize, copy.clone());
            view_model.roles.push(copy);
        }

        ensure_default_roles(&mut view_model.roles);

        for dancer in &snapshot.1 {
            let role = role_map
                .get(&(Rc::as_ptr(&dancer.role) as usize))
                .cloned()
                .or_else(|| {
                    view_model
                        .roles
                        .iter()
                        .find(|candidate| candidate.name.eq_ignore_ascii_case(&dancer.role.name))
                        .cloned()
                })
                .or_else(|| view_model.roles.first().cloned())
                .unwrap_or_else(|| Rc::new(default_role("Dame")));

            let copy = Rc::new(DancerModel {
                dancer_id: dancer.dancer_id,
                role,
                name: dancer.name.clone(),
                shortcut: dancer.shortcut.clone(),
                color: dancer.color.clone(),
                icon: dancer.icon.clone(),
            });
            view_model.dancers.push(copy);
        }

        view_model.selected_dancer = view_model.dancers.first().cloned();
    }
}

impl Behavior<DancerSettingsViewModel> for LoadDancerSettingsBehavior {
    fn activate(
        &self,
        view_model: &mut DancerSettingsViewModel,
        disposables: &mut CompositeDisposable,
    ) {
        BehaviorLog::behavior_activated("LoadDancerSettingsBehavior", "DancerSettingsViewModel");
        self.load(view_model);
        let _ = self.selection_sender.send(DancerSelectionCommand::Refresh);
        let _ = self.swap_selection_sender.send(UpdateSwapSelectionCommand);

        let Some(view_model_handle) = view_model.self_handle().and_then(|handle| handle.upgrade())
        else {
            return;
        };

        let behavior = Self {
            global_state: Rc::clone(&self.global_state),
            reload_receiver: self.reload_receiver.clone(),
            selection_sender: self.selection_sender.clone(),
            swap_selection_sender: self.swap_selection_sender.clone(),
        };
        let reload_receiver = self.reload_receiver.clone();
        let timer = slint::Timer::default();
        timer.start(TimerMode::Repeated, Duration::from_millis(16), move || {
            while reload_receiver.try_recv().is_ok() {
                let mut view_model = view_model_handle.borrow_mut();
                behavior.load(&mut view_model);
                let _ = behavior.selection_sender.send(DancerSelectionCommand::Refresh);
                let _ = behavior
                    .swap_selection_sender
                    .send(UpdateSwapSelectionCommand);
            }
        });
        disposables.add(Box::new(TimerDisposable::new(timer)));
    }
}
