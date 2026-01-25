use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use choreo_models::{DancerModel, RoleModel};
use nject::injectable;

use crate::behavior::{Behavior, CompositeDisposable};
use crate::global::GlobalStateModel;
use crate::logging::BehaviorLog;

use super::mapper::{default_role, ensure_default_roles};
use super::dancer_settings_view_model::DancerSettingsViewModel;

#[injectable]
#[inject(|global_state: Rc<RefCell<GlobalStateModel>>| Self::new(global_state))]
pub struct LoadDancerSettingsBehavior {
    global_state: Rc<RefCell<GlobalStateModel>>,
}

impl LoadDancerSettingsBehavior {
    pub fn new(global_state: Rc<RefCell<GlobalStateModel>>) -> Self {
        Self { global_state }
    }

    pub fn load(&self, view_model: &mut DancerSettingsViewModel) {
        view_model.roles.clear();
        view_model.dancers.clear();

        let mut role_map: HashMap<usize, Rc<RoleModel>> = HashMap::new();
        let global_state = self.global_state.borrow();
        for role in &global_state.choreography.roles {
            let copy = Rc::new(RoleModel {
                name: role.name.clone(),
                color: role.color.clone(),
                z_index: role.z_index,
            });
            role_map.insert(Rc::as_ptr(role) as usize, copy.clone());
            view_model.roles.push(copy);
        }

        ensure_default_roles(&mut view_model.roles);

        for dancer in &global_state.choreography.dancers {
            let role = role_map
                .get(&(Rc::as_ptr(&dancer.role) as usize))
                .cloned()
                .or_else(|| {
                    view_model.roles.iter().find(|candidate| {
                        candidate.name.eq_ignore_ascii_case(&dancer.role.name)
                    }).cloned()
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
        BehaviorLog::behavior_activated(
            "LoadDancerSettingsBehavior",
            "DancerSettingsViewModel",
        );
        self.load(view_model);
    }
}

