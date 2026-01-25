use std::cell::RefCell;
use std::rc::Rc;

use crossbeam_channel::{Receiver, Sender};
use choreo_models::{Colors, SceneModel, SettingsPreferenceKeys};
use choreo_state_machine::{
    ApplicationStateMachine, ApplicationTrigger, PlacePositionsCompletedTrigger,
    PlacePositionsStartedTrigger,
};

use crate::global::GlobalStateModel;
use crate::preferences::Preferences;

use super::mapper::SceneMapper;
use super::view_model::{
    SceneSelectedEvent, SceneViewModel, ScenesPaneViewModel, SelectedSceneChangedEvent,
};

pub struct SelectSceneFromAudioPositionBehavior;

impl SelectSceneFromAudioPositionBehavior {
    pub fn update_selection<P: Preferences>(
        view_model: &mut ScenesPaneViewModel<P>,
        position_seconds: f64,
    ) {
        if view_model.scenes.is_empty() {
            return;
        }

        let mut first_index = None;
        for (index, scene) in view_model.scenes.iter().enumerate() {
            if scene.timestamp.is_some() {
                first_index = Some(index);
                break;
            }
        }

        let Some(start_index) = first_index else {
            return;
        };

        let first_scene = &view_model.scenes[start_index];
        let Some(first_timestamp) = first_scene.timestamp else {
            return;
        };

        if position_seconds < first_timestamp {
            view_model.set_selected_scene(Some(first_scene.clone()));
            return;
        }

        for index in start_index..view_model.scenes.len() {
            let current_scene = &view_model.scenes[index];
            let Some(current_timestamp) = current_scene.timestamp else {
                continue;
            };

            let next_index = index + 1;
            if next_index >= view_model.scenes.len() {
                return;
            }

            let next_scene = &view_model.scenes[next_index];
            let Some(next_timestamp) = next_scene.timestamp else {
                continue;
            };

            if position_seconds >= current_timestamp && position_seconds < next_timestamp {
                view_model.set_selected_scene(Some(current_scene.clone()));
                return;
            }
        }
    }
}

pub struct ShowSceneTimestampsBehavior {
    global_state: Rc<RefCell<GlobalStateModel>>,
}

impl ShowSceneTimestampsBehavior {
    pub fn new(global_state: Rc<RefCell<GlobalStateModel>>) -> Self {
        Self { global_state }
    }

    pub fn update_from_choreography<P: Preferences>(&self, view_model: &mut ScenesPaneViewModel<P>) {
        view_model.show_timestamps = self.global_state.borrow().choreography.settings.show_timestamps;
    }
}

pub struct ApplyPlacementModeBehavior {
    global_state: Rc<RefCell<GlobalStateModel>>,
    state_machine: Option<Rc<RefCell<ApplicationStateMachine>>>,
    receiver: Receiver<SelectedSceneChangedEvent>,
}

impl ApplyPlacementModeBehavior {
    pub fn new(
        global_state: Rc<RefCell<GlobalStateModel>>,
        state_machine: Option<Rc<RefCell<ApplicationStateMachine>>>,
        receiver: Receiver<SelectedSceneChangedEvent>,
    ) -> Self {
        Self {
            global_state,
            state_machine,
            receiver,
        }
    }

    pub fn try_handle(&mut self) -> bool {
        match self.receiver.try_recv() {
            Ok(event) => {
                self.apply_for_scene(event.selected_scene.as_ref());
                true
            }
            Err(_) => false,
        }
    }

    fn apply_for_scene(&self, selected_scene: Option<&SceneViewModel>) {
        if selected_scene.is_none() {
            let mut global_state = self.global_state.borrow_mut();
            global_state.is_place_mode = false;
            if let Some(state_machine) = &self.state_machine {
                state_machine
                    .borrow_mut()
                    .try_apply(&PlacePositionsCompletedTrigger);
            }
            return;
        }

        let selected_scene = selected_scene.unwrap();
        let (dancer_count, position_count, scene_id) = {
            let global_state = self.global_state.borrow();
            let choreography = &global_state.choreography;
            let position_count = choreography
                .scenes
                .iter()
                .find(|scene| scene.scene_id == selected_scene.scene_id)
                .map(|scene| scene.positions.len())
                .unwrap_or_default();
            (choreography.dancers.len(), position_count, selected_scene.scene_id)
        };

        let should_place = dancer_count > 0 && position_count < dancer_count;
        let mut global_state = self.global_state.borrow_mut();
        global_state.is_place_mode = should_place;
        if let Some(state_machine) = &self.state_machine {
            let trigger: &dyn ApplicationTrigger = if should_place {
                &PlacePositionsStartedTrigger
            } else {
                &PlacePositionsCompletedTrigger
            };
            state_machine.borrow_mut().try_apply(trigger);
        }

        if should_place
            && let Some(scene) = global_state
                .choreography
                .scenes
                .iter_mut()
                .find(|scene| scene.scene_id == scene_id)
        {
            for position in &mut scene.positions {
                position.dancer = None;
            }
        }
    }
}

pub struct FilterScenesBehavior;

impl FilterScenesBehavior {
    pub fn apply<P: Preferences>(view_model: &mut ScenesPaneViewModel<P>) {
        view_model.refresh_scenes();
    }
}

pub struct InsertSceneBehavior {
    global_state: Rc<RefCell<GlobalStateModel>>,
}

impl InsertSceneBehavior {
    pub fn new(global_state: Rc<RefCell<GlobalStateModel>>) -> Self {
        Self { global_state }
    }

    pub fn insert_scene<P: Preferences>(
        &self,
        view_model: &mut ScenesPaneViewModel<P>,
        insert_after: bool,
    ) {
        let selected_scene_id = self
            .global_state
            .borrow()
            .selected_scene
            .as_ref()
            .map(|scene| scene.scene_id);

        let mut global_state = self.global_state.borrow_mut();
        let scenes = &mut global_state.scenes;

        let insert_index = match selected_scene_id {
            None => scenes.len(),
            Some(selected_id) => scenes
                .iter()
                .position(|scene| scene.scene_id == selected_id)
                .map(|index| if insert_after { index + 1 } else { index })
                .unwrap_or_else(|| scenes.len()),
        };

        let name = build_scene_name(scenes);
        let next_scene_id = next_scene_id(scenes);
        let new_view_model = SceneViewModel::new(next_scene_id, name, Colors::transparent());

        scenes.insert(insert_index, new_view_model.clone());
        global_state.selected_scene = Some(new_view_model.clone());

        let model_scene = SceneModel {
            scene_id: new_view_model.scene_id,
            positions: Vec::new(),
            name: new_view_model.name.clone(),
            text: None,
            fixed_positions: false,
            timestamp: None,
            variation_depth: 0,
            variations: Vec::new(),
            current_variation: Vec::new(),
            color: Colors::transparent(),
        };
        let model_insert = insert_index.min(global_state.choreography.scenes.len());
        global_state.choreography.scenes.insert(model_insert, model_scene);

        view_model.refresh_scenes();
        view_model.set_selected_scene(global_state.selected_scene.clone());
    }
}

fn build_scene_name(scenes: &[SceneViewModel]) -> String {
    const BASE_NAME: &str = "New Scene";
    if scenes.iter().all(|scene| scene.name != BASE_NAME) {
        return BASE_NAME.to_string();
    }

    let mut suffix = 2;
    loop {
        let candidate = format!("{BASE_NAME} {suffix}");
        if scenes.iter().all(|scene| scene.name != candidate) {
            return candidate;
        }
        suffix += 1;
    }
}

fn next_scene_id(scenes: &[SceneViewModel]) -> choreo_master_mobile_json::SceneId {
    let mut next = 0;
    for scene in scenes {
        next = next.max(scene.scene_id.0 as i64);
    }
    choreo_master_mobile_json::SceneId(next.saturating_add(1) as i32)
}

pub struct LoadScenesBehavior {
    global_state: Rc<RefCell<GlobalStateModel>>,
}

impl LoadScenesBehavior {
    pub fn new(global_state: Rc<RefCell<GlobalStateModel>>) -> Self {
        Self { global_state }
    }

    pub fn load<P: Preferences>(&self, view_model: &mut ScenesPaneViewModel<P>) {
        let scenes = {
            let global_state = self.global_state.borrow();
            let mapper = SceneMapper;
            global_state
                .choreography
                .scenes
                .iter()
                .map(|scene| {
                    let mut view_model =
                        SceneViewModel::new(scene.scene_id, scene.name.clone(), scene.color.clone());
                    mapper.map_model_to_view_model(scene, &mut view_model);
                    view_model
                })
                .collect::<Vec<_>>()
        };

        let mut global_state = self.global_state.borrow_mut();
        global_state.scenes = scenes;
        global_state.selected_scene = global_state.scenes.first().cloned();
        view_model.refresh_scenes();
        view_model.set_selected_scene(global_state.selected_scene.clone());
    }
}

pub struct OpenChoreoBehavior<P: Preferences> {
    preferences: P,
}

impl<P: Preferences> OpenChoreoBehavior<P> {
    pub fn new(preferences: P) -> Self {
        Self { preferences }
    }

    pub fn set_last_opened(&self, path: &str) {
        self.preferences.set_string(
            SettingsPreferenceKeys::LAST_OPENED_CHOREO_FILE,
            path.to_string(),
        );
    }
}

pub struct PublishSceneSelectedBehavior {
    sender: Sender<SceneSelectedEvent>,
}

impl PublishSceneSelectedBehavior {
    pub fn new(sender: Sender<SceneSelectedEvent>) -> Self {
        Self { sender }
    }

    pub fn publish(&self, selected_scene: SceneViewModel) {
        let _ = self.sender.send(SceneSelectedEvent { selected_scene });
    }
}

pub struct SaveChoreoBehavior {
    global_state: Rc<RefCell<GlobalStateModel>>,
}

impl SaveChoreoBehavior {
    pub fn new(global_state: Rc<RefCell<GlobalStateModel>>) -> Self {
        Self { global_state }
    }

    pub fn save(&self) {
        let mut global_state = self.global_state.borrow_mut();
        let mapper = SceneMapper;
        let mut scenes = Vec::with_capacity(global_state.scenes.len());
        for scene_view_model in &global_state.scenes {
            let mut model_scene = global_state
                .choreography
                .scenes
                .iter()
                .find(|scene| scene.scene_id == scene_view_model.scene_id)
                .cloned()
                .or_else(|| {
                    global_state
                        .choreography
                        .scenes
                        .iter()
                        .find(|scene| scene.name == scene_view_model.name)
                        .cloned()
                })
                .unwrap_or_else(|| SceneModel {
                    scene_id: scene_view_model.scene_id,
                    positions: Vec::new(),
                    name: scene_view_model.name.clone(),
                    text: None,
                    fixed_positions: false,
                    timestamp: None,
                    variation_depth: 0,
                    variations: Vec::new(),
                    current_variation: Vec::new(),
                    color: Colors::transparent(),
                });

            mapper.map_view_model_to_model(scene_view_model, &mut model_scene);
            scenes.push(model_scene);
        }

        global_state.choreography.scenes = scenes;
    }
}

pub struct SelectSceneBehavior {
    receiver: Receiver<SceneSelectedEvent>,
    sender: Sender<SelectedSceneChangedEvent>,
}

impl SelectSceneBehavior {
    pub fn new(
        receiver: Receiver<SceneSelectedEvent>,
        sender: Sender<SelectedSceneChangedEvent>,
    ) -> Self {
        Self { receiver, sender }
    }

    pub fn try_handle<P: Preferences>(&self, view_model: &mut ScenesPaneViewModel<P>) -> bool {
        match self.receiver.try_recv() {
            Ok(event) => {
                view_model.set_selected_scene(Some(event.selected_scene.clone()));
                let _ = self.sender.send(SelectedSceneChangedEvent {
                    selected_scene: view_model.selected_scene().clone(),
                });
                true
            }
            Err(_) => false,
        }
    }
}
