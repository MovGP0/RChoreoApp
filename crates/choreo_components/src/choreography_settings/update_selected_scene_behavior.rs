use std::rc::Rc;

use crate::behavior::{Behavior, CompositeDisposable};
use crate::global::GlobalStateActor;
use crate::logging::BehaviorLog;

use super::mapper::{find_scene_mut, format_seconds, normalize_text, update_selected_scene};
use super::choreography_settings_view_model::ChoreographySettingsViewModel;
use nject::injectable;

#[injectable]
pub struct UpdateSelectedSceneBehavior {
    global_state: Rc<GlobalStateActor>,
    is_updating: bool,
}

impl UpdateSelectedSceneBehavior {
    pub fn new(global_state: Rc<GlobalStateActor>) -> Self {
        Self {
            global_state,
            is_updating: false,
        }
    }

    pub fn sync_from_selected(&mut self, view_model: &mut ChoreographySettingsViewModel) {
        let Some(snapshot) = self.global_state.try_with_state(|global_state| {
            (
                global_state.selected_scene.clone(),
                global_state.choreography.clone(),
            )
        }) else {
            return;
        };
        self.is_updating = true;
        update_selected_scene(
            view_model,
            &snapshot.0,
            &snapshot.1,
        );
        self.is_updating = false;
    }

    pub fn update_scene_name(&mut self, view_model: &mut ChoreographySettingsViewModel) {
        if self.is_updating {
            return;
        }
        let _ = self.global_state.try_update(|global_state| {
            let scene_id = global_state.selected_scene.as_ref().map(|scene| scene.scene_id);
            if let Some(scene) = global_state.selected_scene.as_mut() {
                scene.name = view_model.scene_name.clone();
            }
            if let Some(model_scene) = find_scene_mut(&mut global_state.choreography, scene_id) {
                model_scene.name = view_model.scene_name.clone();
            }
        });
    }

    pub fn update_scene_text(&mut self, view_model: &mut ChoreographySettingsViewModel) {
        if self.is_updating {
            return;
        }
        let _ = self.global_state.try_update(|global_state| {
            let scene_id = global_state.selected_scene.as_ref().map(|scene| scene.scene_id);
            if let Some(scene) = global_state.selected_scene.as_mut() {
                scene.text = view_model.scene_text.clone();
            }
            if let Some(model_scene) = find_scene_mut(&mut global_state.choreography, scene_id) {
                model_scene.text = normalize_text(&view_model.scene_text);
            }
        });
    }

    pub fn update_scene_fixed_positions(&mut self, view_model: &mut ChoreographySettingsViewModel) {
        if self.is_updating {
            return;
        }
        let _ = self.global_state.try_update(|global_state| {
            let scene_id = global_state.selected_scene.as_ref().map(|scene| scene.scene_id);
            if let Some(scene) = global_state.selected_scene.as_mut() {
                scene.fixed_positions = view_model.scene_fixed_positions;
            }
            if let Some(model_scene) = find_scene_mut(&mut global_state.choreography, scene_id) {
                model_scene.fixed_positions = view_model.scene_fixed_positions;
            }
        });
    }

    pub fn update_scene_color(&mut self, view_model: &mut ChoreographySettingsViewModel) {
        if self.is_updating {
            return;
        }
        let _ = self.global_state.try_update(|global_state| {
            let scene_id = global_state.selected_scene.as_ref().map(|scene| scene.scene_id);
            if let Some(scene) = global_state.selected_scene.as_mut() {
                scene.color = view_model.scene_color.clone();
            }
            if let Some(model_scene) = find_scene_mut(&mut global_state.choreography, scene_id) {
                model_scene.color = view_model.scene_color.clone();
            }
        });
    }

    pub fn update_scene_timestamp(&mut self, view_model: &mut ChoreographySettingsViewModel) {
        if self.is_updating {
            return;
        }
        let _ = self.global_state.try_update(|global_state| {
            let scene_id = global_state.selected_scene.as_ref().map(|scene| scene.scene_id);
            let timestamp = if view_model.scene_has_timestamp {
                Some(format_seconds(view_model.scene_timestamp_seconds))
            } else {
                None
            };
            if let Some(scene) = global_state.selected_scene.as_mut() {
                scene.timestamp = if view_model.scene_has_timestamp {
                    Some(view_model.scene_timestamp_seconds)
                } else {
                    None
                };
            }
            if let Some(model_scene) = find_scene_mut(&mut global_state.choreography, scene_id) {
                model_scene.timestamp = timestamp;
            }
        });
    }
}

impl Behavior<ChoreographySettingsViewModel> for UpdateSelectedSceneBehavior {
    fn activate(
        &self,
        _view_model: &mut ChoreographySettingsViewModel,
        _disposables: &mut CompositeDisposable,
    ) {
        BehaviorLog::behavior_activated(
            "UpdateSelectedSceneBehavior",
            "ChoreographySettingsViewModel",
        );
    }
}


