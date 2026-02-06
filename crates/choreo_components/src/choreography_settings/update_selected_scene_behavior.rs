use std::cell::Cell;
use std::rc::Rc;
use std::time::Duration;

use crossbeam_channel::Receiver;
use nject::injectable;
use slint::TimerMode;

use crate::behavior::{Behavior, CompositeDisposable, TimerDisposable};
use crate::global::GlobalStateActor;
use crate::logging::BehaviorLog;

use super::choreography_settings_view_model::ChoreographySettingsViewModel;
use super::mapper::{find_scene_mut, format_seconds, normalize_text, update_selected_scene};
use super::messages::UpdateSelectedSceneCommand;

#[injectable]
pub struct UpdateSelectedSceneBehavior {
    global_state: Rc<GlobalStateActor>,
    receiver: Option<Receiver<UpdateSelectedSceneCommand>>,
    is_updating: Cell<bool>,
}

impl UpdateSelectedSceneBehavior {
    pub fn new(global_state: Rc<GlobalStateActor>) -> Self {
        Self {
            global_state,
            receiver: None,
            is_updating: Cell::new(false),
        }
    }

    pub fn new_with_receiver(
        global_state: Rc<GlobalStateActor>,
        receiver: Receiver<UpdateSelectedSceneCommand>,
    ) -> Self {
        Self {
            global_state,
            receiver: Some(receiver),
            is_updating: Cell::new(false),
        }
    }

    fn sync_from_selected(&self, view_model: &mut ChoreographySettingsViewModel) {
        let Some(snapshot) = self.global_state.try_with_state(|global_state| {
            (
                global_state.selected_scene.clone(),
                global_state.choreography.clone(),
            )
        }) else {
            return;
        };
        self.is_updating.set(true);
        update_selected_scene(view_model, &snapshot.0, &snapshot.1);
        self.is_updating.set(false);
    }

    fn update_scene_name(&self, value: &str) {
        if self.is_updating.get() {
            return;
        }
        let _ = self.global_state.try_update(|global_state| {
            let scene_id = global_state.selected_scene.as_ref().map(|scene| scene.scene_id);
            if let Some(scene) = global_state.selected_scene.as_mut() {
                scene.name = value.to_string();
            }
            if let Some(model_scene) = find_scene_mut(&mut global_state.choreography, scene_id) {
                model_scene.name = value.to_string();
            }
        });
    }

    fn update_scene_text(&self, value: &str) {
        if self.is_updating.get() {
            return;
        }
        let _ = self.global_state.try_update(|global_state| {
            let scene_id = global_state.selected_scene.as_ref().map(|scene| scene.scene_id);
            if let Some(scene) = global_state.selected_scene.as_mut() {
                scene.text = value.to_string();
            }
            if let Some(model_scene) = find_scene_mut(&mut global_state.choreography, scene_id) {
                model_scene.text = normalize_text(value);
            }
        });
    }

    fn update_scene_fixed_positions(&self, value: bool) {
        if self.is_updating.get() {
            return;
        }
        let _ = self.global_state.try_update(|global_state| {
            let scene_id = global_state.selected_scene.as_ref().map(|scene| scene.scene_id);
            if let Some(scene) = global_state.selected_scene.as_mut() {
                scene.fixed_positions = value;
            }
            if let Some(model_scene) = find_scene_mut(&mut global_state.choreography, scene_id) {
                model_scene.fixed_positions = value;
            }
        });
    }

    fn update_scene_color(&self, value: choreo_master_mobile_json::Color) {
        if self.is_updating.get() {
            return;
        }
        let _ = self.global_state.try_update(|global_state| {
            let scene_id = global_state.selected_scene.as_ref().map(|scene| scene.scene_id);
            if let Some(scene) = global_state.selected_scene.as_mut() {
                scene.color = value.clone();
            }
            if let Some(model_scene) = find_scene_mut(&mut global_state.choreography, scene_id) {
                model_scene.color = value;
            }
        });
    }

    fn update_scene_timestamp(&self, has_timestamp: bool, seconds: f64) {
        if self.is_updating.get() {
            return;
        }
        let _ = self.global_state.try_update(|global_state| {
            let scene_id = global_state.selected_scene.as_ref().map(|scene| scene.scene_id);
            let timestamp = if has_timestamp {
                Some(format_seconds(seconds))
            } else {
                None
            };
            if let Some(scene) = global_state.selected_scene.as_mut() {
                scene.timestamp = if has_timestamp {
                    Some(seconds)
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
        view_model: &mut ChoreographySettingsViewModel,
        disposables: &mut CompositeDisposable,
    ) {
        BehaviorLog::behavior_activated(
            "UpdateSelectedSceneBehavior",
            "ChoreographySettingsViewModel",
        );
        let Some(receiver) = self.receiver.clone() else {
            return;
        };
        let Some(view_model_handle) = view_model.self_handle().and_then(|handle| handle.upgrade())
        else {
            return;
        };
        let behavior = Self {
            global_state: self.global_state.clone(),
            receiver: None,
            is_updating: Cell::new(false),
        };
        let timer = slint::Timer::default();
        timer.start(TimerMode::Repeated, Duration::from_millis(16), move || {
            while let Ok(command) = receiver.try_recv() {
                match command {
                    UpdateSelectedSceneCommand::SyncFromSelected => {
                        let mut view_model = view_model_handle.borrow_mut();
                        behavior.sync_from_selected(&mut view_model);
                        view_model.notify_changed();
                    }
                    UpdateSelectedSceneCommand::SceneName(value) => {
                        behavior.update_scene_name(&value);
                    }
                    UpdateSelectedSceneCommand::SceneText(value) => {
                        behavior.update_scene_text(&value);
                    }
                    UpdateSelectedSceneCommand::SceneFixedPositions(value) => {
                        behavior.update_scene_fixed_positions(value);
                    }
                    UpdateSelectedSceneCommand::SceneColor(value) => {
                        behavior.update_scene_color(value);
                    }
                    UpdateSelectedSceneCommand::SceneTimestamp {
                        has_timestamp,
                        seconds,
                    } => {
                        behavior.update_scene_timestamp(has_timestamp, seconds);
                    }
                }
            }
        });
        disposables.add(Box::new(TimerDisposable::new(timer)));
    }
}
