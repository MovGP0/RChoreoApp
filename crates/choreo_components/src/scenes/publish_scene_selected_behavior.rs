use crossbeam_channel::Sender;
use nject::injectable;

use crate::behavior::{Behavior, CompositeDisposable};
use crate::logging::BehaviorLog;
use super::messages::SceneSelectedEvent;
use super::scenes_view_model::{SceneViewModel, ScenesPaneViewModel};

#[injectable]
#[inject(|sender: Sender<SceneSelectedEvent>| Self { sender })]
pub struct PublishSceneSelectedBehavior {
    sender: Sender<SceneSelectedEvent>,
}

impl PublishSceneSelectedBehavior {
    pub fn new(sender: Sender<SceneSelectedEvent>) -> Self {
        Self { sender }
    }

    pub fn publish(&self, selected_scene: SceneViewModel) {
        let _ = self.sender.send(SceneSelectedEvent { selected_scene });
    }
}

impl Behavior<ScenesPaneViewModel> for PublishSceneSelectedBehavior {
    fn activate(&self, _view_model: &mut ScenesPaneViewModel, _disposables: &mut CompositeDisposable) {
        BehaviorLog::behavior_activated("PublishSceneSelectedBehavior", "ScenesPaneViewModel");
    }
}
