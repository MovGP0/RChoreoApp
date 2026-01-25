use crossbeam_channel::{Receiver, Sender};

use crate::preferences::Preferences;

use super::messages::{SceneSelectedEvent, SelectedSceneChangedEvent};
use super::scenes_view_model::ScenesPaneViewModel;

pub struct SelectSceneBehavior {
    receiver: Receiver<SceneSelectedEvent>,
    sender: Sender<SelectedSceneChangedEvent>,
}

impl SelectSceneBehavior {
    pub fn new(
        receiver: Receiver<SceneSelectedEvent>,
        sender: Sender<SelectedSceneChangedEvent>,
    ) -> Self {
        Self { receiver, sender }
    }

    pub fn try_handle<P: Preferences>(&self, view_model: &mut ScenesPaneViewModel<P>) -> bool {
        match self.receiver.try_recv() {
            Ok(event) => {
                view_model.set_selected_scene(Some(event.selected_scene.clone()));
                let _ = self.sender.send(SelectedSceneChangedEvent {
                    selected_scene: view_model.selected_scene().clone(),
                });
                true
            }
            Err(_) => false,
        }
    }
}
