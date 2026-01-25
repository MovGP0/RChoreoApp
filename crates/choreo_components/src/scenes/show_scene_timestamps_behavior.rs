use std::cell::RefCell;
use std::rc::Rc;

use crate::global::GlobalStateModel;
use crate::preferences::Preferences;
use nject::injectable;

use super::scenes_view_model::ScenesPaneViewModel;

#[injectable]
#[inject(|global_state: Rc<RefCell<GlobalStateModel>>| Self::new(global_state))]
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
