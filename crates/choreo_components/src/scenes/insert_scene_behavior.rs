use std::cell::RefCell;
use std::rc::Rc;

use choreo_models::{Colors, SceneModel};
use nject::injectable;

use crate::behavior::{Behavior, CompositeDisposable};
use crate::global::GlobalStateModel;
use crate::logging::BehaviorLog;
use super::mapper::{build_scene_name, next_scene_id};
use super::scenes_view_model::{SceneViewModel, ScenesPaneViewModel};

#[injectable]
#[inject(|global_state: Rc<RefCell<GlobalStateModel>>| Self::new(global_state))]
pub struct InsertSceneBehavior {
    global_state: Rc<RefCell<GlobalStateModel>>,
}

impl InsertSceneBehavior {
    pub fn new(global_state: Rc<RefCell<GlobalStateModel>>) -> Self {
        Self { global_state }
    }

    fn insert_scene(&self, view_model: &mut ScenesPaneViewModel, insert_after: bool) {
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

impl Behavior<ScenesPaneViewModel> for InsertSceneBehavior {
    fn activate(&self, view_model: &mut ScenesPaneViewModel, _disposables: &mut CompositeDisposable) {
        BehaviorLog::behavior_activated("InsertSceneBehavior", "ScenesPaneViewModel");
        let before_behavior = self.global_state.clone();
        view_model.set_add_scene_before_handler(Some(Rc::new(move |view_model| {
            let behavior = InsertSceneBehavior::new(before_behavior.clone());
            behavior.insert_scene(view_model, false);
            view_model.update_can_save();
        })));

        let after_behavior = self.global_state.clone();
        view_model.set_add_scene_after_handler(Some(Rc::new(move |view_model| {
            let behavior = InsertSceneBehavior::new(after_behavior.clone());
            behavior.insert_scene(view_model, true);
            view_model.update_can_save();
        })));
    }
}
