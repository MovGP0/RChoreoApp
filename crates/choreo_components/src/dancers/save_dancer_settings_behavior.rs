use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use choreo_master_mobile_json::DancerId;
use choreo_models::DancerModel;

use crate::behavior::{Behavior, CompositeDisposable};
use crate::global::GlobalStateModel;
use crate::logging::BehaviorLog;

use super::mapper::update_scene_dancers;
use super::dancer_settings_view_model::DancerSettingsViewModel;

pub struct SaveDancerSettingsBehavior {
    global_state: Rc<RefCell<GlobalStateModel>>,
}

impl SaveDancerSettingsBehavior {
    pub fn new(global_state: Rc<RefCell<GlobalStateModel>>) -> Self {
        Self { global_state }
    }

    pub fn apply_changes(&mut self, view_model: &DancerSettingsViewModel) {
        let mut global_state = self.global_state.borrow_mut();
        let choreography = &mut global_state.choreography;
        choreography.roles.clear();
        choreography
            .roles
            .extend(view_model.roles.iter().cloned());

        choreography.dancers.clear();
        choreography
            .dancers
            .extend(view_model.dancers.iter().cloned());

        let dancer_map: HashMap<DancerId, Rc<DancerModel>> = view_model
            .dancers
            .iter()
            .map(|dancer| (dancer.dancer_id, dancer.clone()))
            .collect();

        for scene in &mut choreography.scenes {
            update_scene_dancers(scene, &dancer_map);
        }
    }
}

impl Behavior<DancerSettingsViewModel> for SaveDancerSettingsBehavior {
    fn activate(
        &self,
        _view_model: &mut DancerSettingsViewModel,
        _disposables: &mut CompositeDisposable,
    ) {
        BehaviorLog::behavior_activated(
            "SaveDancerSettingsBehavior",
            "DancerSettingsViewModel",
        );
    }
}

