use std::cell::RefCell;
use std::rc::Rc;

use crate::global::GlobalStateModel;
use nject::injectable;

use crate::behavior::{Behavior, CompositeDisposable};
use crate::logging::BehaviorLog;

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

    pub fn update_from_choreography(&self, view_model: &mut ScenesPaneViewModel) {
        view_model.show_timestamps = self.global_state.borrow().choreography.settings.show_timestamps;
    }
}

impl Behavior<ScenesPaneViewModel> for ShowSceneTimestampsBehavior {
    fn initialize(&self, view_model: &mut ScenesPaneViewModel, _disposables: &mut CompositeDisposable) {
        BehaviorLog::behavior_activated("ShowSceneTimestampsBehavior", "ScenesPaneViewModel");
        self.update_from_choreography(view_model);
    }
}
