use std::cell::RefCell;
use std::rc::Rc;

use crossbeam_channel::Sender;
use choreo_master_mobile_json::{Color, SceneId};
use choreo_models::{PositionModel, SceneModel, SettingsPreferenceKeys};
use nject::injectable;

use crate::audio_player::HapticFeedback;
use crate::global::GlobalStateModel;
use crate::preferences::Preferences;

use super::messages::{CloseDialogCommand, DialogRequest, ShowDialogCommand};

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


#[injectable]
#[inject(
    |global_state: Rc<RefCell<GlobalStateModel>>,
     preferences: P,
     show_dialog_sender: Sender<ShowDialogCommand>,
     close_dialog_sender: Sender<CloseDialogCommand>,
     haptic_feedback: Option<Box<dyn HapticFeedback>>| {
        Self::new(
            global_state,
            preferences,
            show_dialog_sender,
            close_dialog_sender,
            haptic_feedback,
        )
    }
)]
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



