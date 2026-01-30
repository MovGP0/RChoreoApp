use std::rc::Rc;

use nject::injectable;

use crate::behavior::{Behavior, CompositeDisposable};
use crate::logging::BehaviorLog;
use super::scenes_view_model::ScenesPaneViewModel;

#[injectable]
#[inject(|| Self)]
pub struct FilterScenesBehavior;

impl FilterScenesBehavior {
    pub fn apply(view_model: &mut ScenesPaneViewModel) {
        view_model.refresh_scenes();
    }
}

impl Behavior<ScenesPaneViewModel> for FilterScenesBehavior {
    fn initialize(&self, view_model: &mut ScenesPaneViewModel, _disposables: &mut CompositeDisposable) {
        BehaviorLog::behavior_activated("FilterScenesBehavior", "ScenesPaneViewModel");
        view_model.set_update_search_text_handler(Some(Rc::new(|view_model| {
            FilterScenesBehavior::apply(view_model);
        })));
    }
}
