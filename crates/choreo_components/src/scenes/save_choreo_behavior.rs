use std::path::Path;
use std::rc::Rc;

use choreo_master_mobile_json::export_to_file;
use choreo_models::{ChoreographyModelMapper, Colors, SceneModel, SettingsPreferenceKeys};
use nject::injectable;

use crate::behavior::{Behavior, CompositeDisposable};
use crate::global::GlobalStateActor;
use crate::logging::BehaviorLog;
use crate::observability::start_internal_span;
use crate::preferences::Preferences;
use crate::time::SystemClock;

use super::mapper::SceneMapper;
use super::scenes_view_model::ScenesPaneViewModel;

#[derive(Clone)]
#[injectable]
#[inject(|global_state: Rc<GlobalStateActor>, preferences: Rc<dyn Preferences>| Self::new(global_state, preferences))]
pub struct SaveChoreoBehavior {
    global_state: Rc<GlobalStateActor>,
    preferences: Rc<dyn Preferences>,
}

impl SaveChoreoBehavior {
    pub fn new(global_state: Rc<GlobalStateActor>, preferences: Rc<dyn Preferences>) -> Self {
        Self {
            global_state,
            preferences,
        }
    }

    fn save(&self, view_model: &mut ScenesPaneViewModel) {
        let mut span = start_internal_span("scenes.save_choreo", None);
        let path = self
            .preferences
            .get_string(SettingsPreferenceKeys::LAST_OPENED_CHOREO_FILE, "");
        if path.trim().is_empty() {
            span.set_bool_attribute("choreo.success", false);
            return;
        }

        let path = Path::new(&path);
        span.set_string_attribute("choreo.file.path", path.to_string_lossy().into_owned());
        if !path.exists() {
            span.set_bool_attribute("choreo.success", false);
            return;
        }

        let mut json_model = None;
        let updated = self.global_state.try_update(|global_state| {
            let mapper = SceneMapper;
            let mut scenes = Vec::with_capacity(global_state.scenes.len());
            span.set_f64_attribute("choreo.scenes.count", global_state.scenes.len() as f64);
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
            global_state.choreography.last_save_date = SystemClock::now_utc();

            let json_mapper = ChoreographyModelMapper;
            json_model = Some(json_mapper.map_to_json(&global_state.choreography));
        });
        if !updated {
            span.set_bool_attribute("choreo.success", false);
            return;
        }
        if let Some(json_model) = json_model {
            let _ = export_to_file(path, &json_model);
        }
        span.set_bool_attribute("choreo.success", true);
        view_model.update_can_save();
    }
}

impl Behavior<ScenesPaneViewModel> for SaveChoreoBehavior {
    fn activate(
        &self,
        view_model: &mut ScenesPaneViewModel,
        _disposables: &mut CompositeDisposable,
    ) {
        BehaviorLog::behavior_activated("SaveChoreoBehavior", "ScenesPaneViewModel");
        let behavior = self.clone();
        view_model.set_save_choreo_handler(Some(Rc::new(move |view_model| {
            behavior.save(view_model);
        })));
    }
}
