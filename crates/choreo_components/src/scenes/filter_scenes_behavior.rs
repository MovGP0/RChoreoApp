use std::rc::Rc;

use nject::injectable;

use super::scenes_view_model::ScenesPaneViewModel;
use crate::behavior::{Behavior, CompositeDisposable};
use crate::logging::BehaviorLog;
use crate::observability::start_internal_span;

#[injectable]
#[inject(|| Self)]
pub struct FilterScenesBehavior;

impl FilterScenesBehavior {
    pub fn apply(view_model: &mut ScenesPaneViewModel) {
        let mut span = start_internal_span("scenes.filter_scenes", None);
        span.set_f64_attribute("choreo.scenes.previous_count", view_model.scenes.len() as f64);
        span.set_f64_attribute(
            "choreo.scenes.search_text_length",
            view_model.search_text.len() as f64,
        );
        view_model.refresh_scenes();
        span.set_f64_attribute("choreo.scenes.filtered_count", view_model.scenes.len() as f64);
    }
}

impl Behavior<ScenesPaneViewModel> for FilterScenesBehavior {
    fn activate(
        &self,
        view_model: &mut ScenesPaneViewModel,
        _disposables: &mut CompositeDisposable,
    ) {
        BehaviorLog::behavior_activated("FilterScenesBehavior", "ScenesPaneViewModel");
        view_model.set_update_search_text_handler(Some(Rc::new(|view_model| {
            FilterScenesBehavior::apply(view_model);
        })));
    }
}
