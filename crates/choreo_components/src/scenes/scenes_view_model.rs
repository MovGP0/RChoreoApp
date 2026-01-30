use std::cell::RefCell;
use std::rc::Rc;

use crossbeam_channel::Sender;
use choreo_master_mobile_json::{Color, SceneId};
use choreo_models::{PositionModel, SceneModel, SettingsPreferenceKeys};
use nject::injectable;

use crate::behavior::{Behavior, CompositeDisposable};
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

type SceneActionHandler = Rc<dyn Fn(&mut ScenesPaneViewModel)>;
type SceneSelectHandler = Rc<dyn Fn(&mut ScenesPaneViewModel, usize)>;

#[derive(Clone, Default)]
pub struct ScenesPaneViewModelActions {
    pub add_scene_before: Option<SceneActionHandler>,
    pub add_scene_after: Option<SceneActionHandler>,
    pub update_search_text: Option<SceneActionHandler>,
    pub delete_scene: Option<SceneActionHandler>,
    pub open_choreo: Option<SceneActionHandler>,
    pub save_choreo: Option<SceneActionHandler>,
    pub navigate_to_settings: Option<SceneActionHandler>,
    pub navigate_to_dancer_settings: Option<SceneActionHandler>,
    pub select_scene: Option<SceneSelectHandler>,
}


#[injectable]
#[inject(
    |global_state: Rc<RefCell<GlobalStateModel>>,
     preferences: Rc<dyn Preferences>,
     show_dialog_sender: Sender<ShowDialogCommand>,
     close_dialog_sender: Sender<CloseDialogCommand>,
     haptic_feedback: Option<Box<dyn HapticFeedback>>,
     behaviors: Vec<Box<dyn Behavior<ScenesPaneViewModel>>>| {
        Self::new(
            global_state,
            preferences,
            show_dialog_sender,
            close_dialog_sender,
            haptic_feedback,
            behaviors,
        )
    }
)]
pub struct ScenesPaneViewModel {
    global_state: Rc<RefCell<GlobalStateModel>>,
    show_dialog_sender: Sender<ShowDialogCommand>,
    _close_dialog_sender: Sender<CloseDialogCommand>,
    haptic_feedback: Option<Box<dyn HapticFeedback>>,
    preferences: Rc<dyn Preferences>,
    actions: ScenesPaneViewModelActions,
    on_change: Option<Rc<dyn Fn()>>,
    disposables: CompositeDisposable,
    pub search_text: String,
    pub scenes: Vec<SceneViewModel>,
    pub can_save_choreo: bool,
    pub can_delete_scene: bool,
    pub show_timestamps: bool,
    pub can_navigate_to_settings: bool,
    pub can_navigate_to_dancer_settings: bool,
}

impl ScenesPaneViewModel {
    pub fn new(
        global_state: Rc<RefCell<GlobalStateModel>>,
        preferences: Rc<dyn Preferences>,
        show_dialog_sender: Sender<ShowDialogCommand>,
        close_dialog_sender: Sender<CloseDialogCommand>,
        haptic_feedback: Option<Box<dyn HapticFeedback>>,
        behaviors: Vec<Box<dyn Behavior<ScenesPaneViewModel>>>,
    ) -> Self {
        let mut view_model = Self {
            global_state,
            show_dialog_sender,
            _close_dialog_sender: close_dialog_sender,
            haptic_feedback,
            preferences,
            actions: ScenesPaneViewModelActions::default(),
            on_change: None,
            disposables: CompositeDisposable::new(),
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

        let mut disposables = CompositeDisposable::new();
        for behavior in behaviors.iter() {
            behavior.activate(&mut view_model, &mut disposables);
        }
        view_model.disposables = disposables;
        view_model
    }

    pub fn set_on_change(&mut self, handler: Option<Rc<dyn Fn()>>) {
        self.on_change = handler;
    }

    pub fn set_actions(&mut self, actions: ScenesPaneViewModelActions) {
        self.actions = actions;
    }

    pub fn set_add_scene_before_handler(
        &mut self,
        handler: Option<SceneActionHandler>,
    ) {
        self.actions.add_scene_before = handler;
    }

    pub fn set_add_scene_after_handler(
        &mut self,
        handler: Option<SceneActionHandler>,
    ) {
        self.actions.add_scene_after = handler;
    }

    pub fn set_update_search_text_handler(
        &mut self,
        handler: Option<SceneActionHandler>,
    ) {
        self.actions.update_search_text = handler;
    }

    pub fn set_delete_scene_handler(
        &mut self,
        handler: Option<SceneActionHandler>,
    ) {
        self.actions.delete_scene = handler;
    }

    pub fn set_open_choreo_handler(
        &mut self,
        handler: Option<SceneActionHandler>,
    ) {
        self.actions.open_choreo = handler;
    }

    pub fn set_save_choreo_handler(
        &mut self,
        handler: Option<SceneActionHandler>,
    ) {
        self.actions.save_choreo = handler;
    }

    pub fn set_navigate_to_settings_handler(
        &mut self,
        handler: Option<SceneActionHandler>,
    ) {
        self.actions.navigate_to_settings = handler;
    }

    pub fn set_navigate_to_dancer_settings_handler(
        &mut self,
        handler: Option<SceneActionHandler>,
    ) {
        self.actions.navigate_to_dancer_settings = handler;
    }

    pub fn set_select_scene_handler(
        &mut self,
        handler: Option<SceneSelectHandler>,
    ) {
        self.actions.select_scene = handler;
    }

    pub fn selected_scene(&self) -> Option<SceneViewModel> {
        self.global_state.borrow().selected_scene.clone()
    }

    pub fn set_selected_scene(&mut self, scene: Option<SceneViewModel>) {
        self.global_state.borrow_mut().selected_scene = scene.clone();
        self.can_delete_scene = scene.is_some();
        self.notify_changed();
    }

    pub fn add_scene_before(&mut self) {
        self.perform_click();
        let handler = self.actions.add_scene_before.clone();
        if let Some(handler) = handler {
            handler(self);
        }
        self.notify_changed();
    }

    pub fn add_scene_after(&mut self) {
        self.perform_click();
        let handler = self.actions.add_scene_after.clone();
        if let Some(handler) = handler {
            handler(self);
        }
        self.notify_changed();
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

    pub fn update_search_text(&mut self, value: String) {
        self.search_text = value;
        let handler = self.actions.update_search_text.clone();
        if let Some(handler) = handler {
            handler(self);
        }
        self.notify_changed();
    }

    pub fn navigate_to_settings(&mut self) {
        self.perform_click();
        let handler = self.actions.navigate_to_settings.clone();
        if let Some(handler) = handler {
            handler(self);
        }
        self.notify_changed();
    }

    pub fn navigate_to_dancer_settings(&mut self) {
        self.perform_click();
        let handler = self.actions.navigate_to_dancer_settings.clone();
        if let Some(handler) = handler {
            handler(self);
        }
        self.notify_changed();
    }

    pub fn open_choreo(&mut self) {
        self.perform_click();
        let handler = self.actions.open_choreo.clone();
        if let Some(handler) = handler {
            handler(self);
        }
        self.notify_changed();
    }

    pub fn save_choreo(&mut self) {
        self.perform_click();
        let handler = self.actions.save_choreo.clone();
        if let Some(handler) = handler {
            handler(self);
        }
        self.notify_changed();
    }

    pub fn delete_scene(&mut self) {
        self.perform_click();
        let handler = self.actions.delete_scene.clone();
        if let Some(handler) = handler {
            handler(self);
        }

        let Some(scene) = self.global_state.borrow().selected_scene.clone() else {
            return;
        };

        let command = ShowDialogCommand {
            dialog: DialogRequest::DeleteScene {
                scene_id: scene.scene_id,
            },
        };
        let _ = self.show_dialog_sender.send(command);
        self.notify_changed();
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
        self.notify_changed();
    }

    pub fn select_scene(&mut self, index: usize) {
        let handler = self.actions.select_scene.clone();
        if let Some(handler) = handler {
            handler(self, index);
        } else if let Some(scene) = self.scenes.get(index).cloned() {
            self.set_selected_scene(Some(scene));
        }
        self.notify_changed();
    }

    pub fn notify_changed(&self) {
        if let Some(handler) = self.on_change.as_ref() {
            handler();
        }
    }

    fn perform_click(&self) {
        if let Some(haptic) = &self.haptic_feedback
            && haptic.is_supported()
        {
            haptic.perform_click();
        }
    }

    pub fn dispose(&mut self) {
        self.disposables.dispose_all();
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
