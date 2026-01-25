use std::cell::RefCell;
use std::rc::Rc;

use crossbeam_channel::{Receiver, Sender};
use choreo_models::{Colors, PositionModel, SceneModel, SettingsPreferenceKeys};
use choreo_master_mobile_json::{Color, SceneId};
use choreo_state_machine::{
    ApplicationStateMachine, ApplicationTrigger, PlacePositionsCompletedTrigger,
    PlacePositionsStartedTrigger,
};

use crate::audio_player::HapticFeedback;
use crate::global::GlobalStateModel;
use crate::preferences::Preferences;

#[derive(Debug, Clone, PartialEq)]
pub struct SceneViewModel {
    pub scene_id: SceneId,
    pub name: String,
    pub text: String,
    pub fixed_positions: bool,
    pub timestamp: Option<f64>,
    pub is_selected: bool,
    pub positions: Vec<PositionModel>,
    pub variation_depth: i32,
    pub variations: Vec<Vec<SceneModel>>,
    pub current_variation: Vec<SceneModel>,
    pub color: Color,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DialogRequest {
    DeleteScene { scene_id: SceneId },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShowDialogCommand {
    pub dialog: DialogRequest,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CloseDialogCommand;

#[derive(Debug, Clone, PartialEq)]
pub struct SceneSelectedEvent {
    pub selected_scene: SceneViewModel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CopyScenePositionsDecision {
    CopyPositions,
    KeepPositions,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CopyScenePositionsDecisionEvent {
    pub decision: CopyScenePositionsDecision,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SelectedSceneChangedEvent {
    pub selected_scene: Option<SceneViewModel>,
}

pub struct ScenesPaneViewModel<P: Preferences> {
    global_state: Rc<RefCell<GlobalStateModel>>,
    show_dialog_sender: Sender<ShowDialogCommand>,
    _close_dialog_sender: Sender<CloseDialogCommand>,
    haptic_feedback: Option<Box<dyn HapticFeedback>>,
    preferences: P,
    pub search_text: String,
    pub scenes: Vec<SceneViewModel>,
    pub can_save_choreo: bool,
    pub can_delete_scene: bool,
    pub show_timestamps: bool,
    pub can_navigate_to_settings: bool,
    pub can_navigate_to_dancer_settings: bool,
}

impl<P: Preferences> ScenesPaneViewModel<P> {
    pub fn new(
        global_state: Rc<RefCell<GlobalStateModel>>,
        preferences: P,
        show_dialog_sender: Sender<ShowDialogCommand>,
        close_dialog_sender: Sender<CloseDialogCommand>,
        haptic_feedback: Option<Box<dyn HapticFeedback>>,
    ) -> Self {
        let mut view_model = Self {
            global_state,
            show_dialog_sender,
            _close_dialog_sender: close_dialog_sender,
            haptic_feedback,
            preferences,
            search_text: String::new(),
            scenes: Vec::new(),
            can_save_choreo: false,
            can_delete_scene: false,
            show_timestamps: false,
            can_navigate_to_settings: true,
            can_navigate_to_dancer_settings: true,
        };

        view_model.refresh_scenes();
        view_model.update_can_save();
        view_model
    }

    pub fn selected_scene(&self) -> Option<SceneViewModel> {
        self.global_state.borrow().selected_scene.clone()
    }

    pub fn set_selected_scene(&mut self, scene: Option<SceneViewModel>) {
        self.global_state.borrow_mut().selected_scene = scene.clone();
        self.can_delete_scene = scene.is_some();
    }

    pub fn add_scene_before(&self) {
        self.perform_click();
    }

    pub fn add_scene_after(&self) {
        self.perform_click();
    }

    pub fn refresh_scenes(&mut self) {
        self.scenes.clear();

        let global_state = self.global_state.borrow();
        if self.search_text.trim().is_empty() {
            self.scenes.extend(global_state.scenes.iter().cloned());
            return;
        }

        let search = self.search_text.to_ascii_lowercase();
        self.scenes.extend(
            global_state
                .scenes
                .iter()
                .filter(|scene| scene.name.to_ascii_lowercase().contains(&search))
                .cloned(),
        );
    }

    pub fn navigate_to_settings(&self) {
        self.perform_click();
    }

    pub fn navigate_to_dancer_settings(&self) {
        self.perform_click();
    }

    pub fn open_choreo(&self) {
        self.perform_click();
    }

    pub fn save_choreo(&self) {
        self.perform_click();
    }

    pub fn delete_scene(&self) {
        self.perform_click();

        let Some(scene) = self.global_state.borrow().selected_scene.clone() else {
            return;
        };

        let command = ShowDialogCommand {
            dialog: DialogRequest::DeleteScene {
                scene_id: scene.scene_id,
            },
        };
        let _ = self.show_dialog_sender.send(command);
    }

    pub fn update_can_save(&mut self) {
        let path = self
            .preferences
            .get_string(SettingsPreferenceKeys::LAST_OPENED_CHOREO_FILE, "");
        let has_path = !path.trim().is_empty();
        let has_file = has_path && std::path::Path::new(&path).exists();

        let has_choreo = !self.global_state.borrow().choreography.name.is_empty()
            || !self.global_state.borrow().choreography.scenes.is_empty();
        self.can_save_choreo = has_choreo && has_file;
    }

    fn perform_click(&self) {
        if let Some(haptic) = &self.haptic_feedback
            && haptic.is_supported()
        {
            haptic.perform_click();
        }
    }
}

impl SceneViewModel {
    pub fn new(scene_id: SceneId, name: impl Into<String>, color: Color) -> Self {
        Self {
            scene_id,
            name: name.into(),
            text: String::new(),
            fixed_positions: false,
            timestamp: None,
            is_selected: false,
            positions: Vec::new(),
            variation_depth: 0,
            variations: Vec::new(),
            current_variation: Vec::new(),
            color,
        }
    }
}

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

pub fn as_observable_collection_extended<T>(source: impl IntoIterator<Item = T>) -> Vec<T> {
    source.into_iter().collect()
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

fn next_scene_id(scenes: &[SceneViewModel]) -> SceneId {
    let mut next = 0;
    for scene in scenes {
        next = next.max(scene.scene_id.0 as i64);
    }
    SceneId(next.saturating_add(1) as i32)
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

pub struct SceneMapper;

impl SceneMapper {
    pub fn map_view_model_to_model(&self, source: &SceneViewModel, target: &mut SceneModel) {
        target.scene_id = source.scene_id;
        target.name = source.name.clone();
        target.text = normalize_text(&source.text);
        target.fixed_positions = source.fixed_positions;
        target.timestamp = source.timestamp.map(format_seconds);
        target.variation_depth = source.variation_depth;
        target.color = source.color.clone();

        target.variations = source
            .variations
            .iter()
            .map(|list| clone_scene_list(list))
            .collect();
        target.current_variation = clone_scene_list(&source.current_variation);

        target.positions.clear();
        target.positions.extend(source.positions.iter().cloned());
    }

    pub fn map_model_to_view_model(&self, source: &SceneModel, target: &mut SceneViewModel) {
        target.scene_id = source.scene_id;
        target.name = source.name.clone();
        target.text = source.text.clone().unwrap_or_default();
        target.fixed_positions = source.fixed_positions;
        target.timestamp = source
            .timestamp
            .as_deref()
            .and_then(parse_timestamp_seconds);
        target.variation_depth = source.variation_depth;
        target.color = source.color.clone();

        target.positions.clear();
        target.positions.extend(source.positions.iter().cloned());
        target.variations = source
            .variations
            .iter()
            .map(|list| clone_scene_list(list))
            .collect();
        target.current_variation = clone_scene_list(&source.current_variation);
    }
}

fn clone_scene_list(source: &[SceneModel]) -> Vec<SceneModel> {
    source.iter().map(clone_scene).collect()
}

fn clone_scene(source: &SceneModel) -> SceneModel {
    SceneModel {
        scene_id: source.scene_id,
        positions: source.positions.to_vec(),
        name: source.name.clone(),
        text: source.text.clone(),
        fixed_positions: source.fixed_positions,
        timestamp: source.timestamp.clone(),
        variation_depth: source.variation_depth,
        variations: source
            .variations
            .iter()
            .map(|list| clone_scene_list(list))
            .collect(),
        current_variation: clone_scene_list(&source.current_variation),
        color: source.color.clone(),
    }
}

fn parse_timestamp_seconds(value: &str) -> Option<f64> {
    let value = value.trim();
    if value.is_empty() {
        return None;
    }

    let mut parts = value.split(':').collect::<Vec<_>>();
    if parts.len() > 3 {
        return None;
    }

    let seconds_part = parts.pop()?;
    let minutes_part = parts.pop().unwrap_or("0");
    let hours_part = parts.pop().unwrap_or("0");

    let seconds = seconds_part.parse::<f64>().ok()?;
    let minutes = minutes_part.parse::<f64>().ok()?;
    let hours = hours_part.parse::<f64>().ok()?;

    Some(hours * 3600.0 + minutes * 60.0 + seconds)
}

fn format_seconds(value: f64) -> String {
    let mut text = format!("{value:.3}");
    if let Some(dot) = text.find('.') {
        while text.ends_with('0') {
            text.pop();
        }
        if text.ends_with('.') {
            text.pop();
        }
        if text.len() == dot {
            text.push('0');
        }
    }
    text
}

fn normalize_text(value: &str) -> Option<String> {
    if value.trim().is_empty() {
        None
    } else {
        Some(value.trim().to_string())
    }
}
