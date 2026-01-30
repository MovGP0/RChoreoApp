use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

use choreo_master_mobile_json::export_to_file;
use choreo_models::{ChoreographyModelMapper, Colors, SceneModel, SettingsPreferenceKeys};
use nject::injectable;
use time::OffsetDateTime;

use crate::behavior::{Behavior, CompositeDisposable};
use crate::global::GlobalStateModel;
use crate::logging::BehaviorLog;
use crate::preferences::Preferences;

use super::mapper::SceneMapper;
use super::scenes_view_model::ScenesPaneViewModel;

#[derive(Clone)]
#[injectable]
#[inject(|global_state: Rc<RefCell<GlobalStateModel>>, preferences: Rc<dyn Preferences>| Self::new(global_state, preferences))]
pub struct SaveChoreoBehavior {
    global_state: Rc<RefCell<GlobalStateModel>>,
    preferences: Rc<dyn Preferences>,
}

impl SaveChoreoBehavior {
    pub fn new(global_state: Rc<RefCell<GlobalStateModel>>, preferences: Rc<dyn Preferences>) -> Self {
        Self {
            global_state,
            preferences,
        }
    }

    pub fn save(&self, view_model: &mut ScenesPaneViewModel) {
        let path = self
            .preferences
            .get_string(SettingsPreferenceKeys::LAST_OPENED_CHOREO_FILE, "");
        if path.trim().is_empty() {
            return;
        }

        let path = Path::new(&path);
        if !path.exists() {
            return;
        }

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
        global_state.choreography.last_save_date = OffsetDateTime::now_utc();

        let json_mapper = ChoreographyModelMapper;
        let json_model = json_mapper.map_to_json(&global_state.choreography);
        let _ = export_to_file(path, &json_model);
        view_model.update_can_save();
    }
}

impl Behavior<ScenesPaneViewModel> for SaveChoreoBehavior {
    fn activate(&self, view_model: &mut ScenesPaneViewModel, _disposables: &mut CompositeDisposable) {
        BehaviorLog::behavior_activated("SaveChoreoBehavior", "ScenesPaneViewModel");
        let behavior = self.clone();
        view_model.set_save_choreo_handler(Some(Rc::new(move |view_model| {
            behavior.save(view_model);
        })));
    }
}
