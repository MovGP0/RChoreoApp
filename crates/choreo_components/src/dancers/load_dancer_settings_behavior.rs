use std::collections::HashMap;
use std::rc::Rc;

use choreo_models::{DancerModel, RoleModel};
use crossbeam_channel::Sender;
use nject::injectable;

use crate::behavior::{Behavior, CompositeDisposable};
use crate::global::GlobalStateActor;
use crate::logging::BehaviorLog;

use super::dancer_settings_view_model::DancerSettingsViewModel;
use super::mapper::{default_role, ensure_default_roles};
use super::messages::{DancerSelectionCommand, UpdateSwapSelectionCommand};

#[injectable]
#[inject(
    |global_state: Rc<GlobalStateActor>,
     selection_sender: Sender<DancerSelectionCommand>,
     swap_selection_sender: Sender<UpdateSwapSelectionCommand>| {
        Self::new(global_state, selection_sender, swap_selection_sender)
    }
)]
pub struct LoadDancerSettingsBehavior {
    global_state: Rc<GlobalStateActor>,
    selection_sender: Sender<DancerSelectionCommand>,
    swap_selection_sender: Sender<UpdateSwapSelectionCommand>,
}

impl LoadDancerSettingsBehavior {
    pub(super) fn new(
        global_state: Rc<GlobalStateActor>,
        selection_sender: Sender<DancerSelectionCommand>,
        swap_selection_sender: Sender<UpdateSwapSelectionCommand>,
    ) -> Self {
        Self {
            global_state,
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
        _disposables: &mut CompositeDisposable,
    ) {
        BehaviorLog::behavior_activated("LoadDancerSettingsBehavior", "DancerSettingsViewModel");
        self.load(view_model);
        let _ = self.selection_sender.send(DancerSelectionCommand::Refresh);
        let _ = self.swap_selection_sender.send(UpdateSwapSelectionCommand);
    }
}
