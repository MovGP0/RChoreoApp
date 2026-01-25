use std::cell::RefCell;
use std::rc::Rc;

use crate::behavior::{Behavior, CompositeDisposable};
use crate::global::GlobalStateModel;
use crate::logging::BehaviorLog;

use super::mapper::{map_from_choreography, reset_view_model, update_selected_scene};
use super::choreography_settings_view_model::ChoreographySettingsViewModel;

pub struct LoadChoreographySettingsBehavior {
    global_state: Rc<RefCell<GlobalStateModel>>,
}

impl LoadChoreographySettingsBehavior {
    pub fn new(global_state: Rc<RefCell<GlobalStateModel>>) -> Self {
        Self { global_state }
    }

    pub fn load(&self, view_model: &mut ChoreographySettingsViewModel) {
        let global_state = self.global_state.borrow();
        let choreography = &global_state.choreography;
        if choreography.name.is_empty()
            && choreography.scenes.is_empty()
            && choreography.comment.is_none()
        {
            reset_view_model(view_model);
            return;
        }

        map_from_choreography(choreography, view_model);
        update_selected_scene(view_model, &global_state.selected_scene, choreography);
    }
}

impl Behavior<ChoreographySettingsViewModel> for LoadChoreographySettingsBehavior {
    fn activate(
        &self,
        view_model: &mut ChoreographySettingsViewModel,
        _disposables: &mut CompositeDisposable,
    ) {
        BehaviorLog::behavior_activated(
            "LoadChoreographySettingsBehavior",
            "ChoreographySettingsViewModel",
        );
        self.load(view_model);
    }
}


