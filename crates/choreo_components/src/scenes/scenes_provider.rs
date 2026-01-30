use std::cell::RefCell;
use std::rc::Rc;

use super::{build_scenes_view_model, ScenesDependencies, ScenesPaneViewModel};

pub struct ScenesProvider {
    scenes_view_model: Rc<RefCell<ScenesPaneViewModel>>,
}

impl ScenesProvider {
    pub fn new(deps: ScenesDependencies) -> Self
    {
        let scenes_view_model = Rc::new(RefCell::new(build_scenes_view_model(deps)));
        Self { scenes_view_model }
    }

    pub fn scenes_view_model(&self) -> Rc<RefCell<ScenesPaneViewModel>> {
        Rc::clone(&self.scenes_view_model)
    }
}
