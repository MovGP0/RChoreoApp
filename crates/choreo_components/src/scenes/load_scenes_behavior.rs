use std::cell::RefCell;
use std::rc::Rc;

use crate::global::GlobalStateModel;
use crate::preferences::Preferences;

use super::mapper::SceneMapper;
use super::scenes_view_model::{SceneViewModel, ScenesPaneViewModel};

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
