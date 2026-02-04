use std::rc::Rc;

use crate::behavior::{Behavior, CompositeDisposable};
use crate::global::GlobalStateStore;
use crate::logging::BehaviorLog;

use super::mapper::{map_from_choreography, reset_view_model, update_selected_scene};
use super::choreography_settings_view_model::ChoreographySettingsViewModel;
use nject::injectable;

#[injectable]
pub struct LoadChoreographySettingsBehavior {
    global_state: Rc<GlobalStateStore>,
}

impl LoadChoreographySettingsBehavior {
    pub fn new(global_state: Rc<GlobalStateStore>) -> Self {
        Self { global_state }
    }

    pub fn load(&self, view_model: &mut ChoreographySettingsViewModel) {
        let Some(snapshot) = self.global_state.try_with_state(|global_state| {
            (global_state.choreography.clone(), global_state.selected_scene.clone())
        }) else {
            return;
        };
        let choreography = &snapshot.0;
        if choreography.name.is_empty()
            && choreography.scenes.is_empty()
            && choreography.comment.is_none()
        {
            reset_view_model(view_model);
            return;
        }

        map_from_choreography(choreography, view_model);
        update_selected_scene(view_model, &snapshot.1, choreography);
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


