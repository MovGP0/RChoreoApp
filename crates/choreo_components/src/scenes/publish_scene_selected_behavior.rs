use crossbeam_channel::Sender;
use nject::injectable;

use super::messages::SceneSelectedEvent;
use super::scenes_view_model::SceneViewModel;

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
