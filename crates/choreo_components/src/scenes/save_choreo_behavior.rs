use std::cell::RefCell;
use std::rc::Rc;

use choreo_models::{Colors, SceneModel};
use nject::injectable;

use crate::global::GlobalStateModel;

use super::mapper::SceneMapper;

#[injectable]
#[inject(|global_state: Rc<RefCell<GlobalStateModel>>| Self::new(global_state))]
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
