use std::cell::RefCell;
use std::rc::Rc;

use crossbeam_channel::{Receiver, Sender};
use nject::injectable;

use crate::behavior::{Behavior, CompositeDisposable};
use crate::global::GlobalStateModel;
use crate::logging::BehaviorLog;
use choreo_state_machine::ApplicationStateMachine;

use super::apply_placement_mode_behavior::ApplyPlacementModeBehavior;
use super::messages::{SceneSelectedEvent, SelectedSceneChangedEvent};
use super::publish_scene_selected_behavior::PublishSceneSelectedBehavior;
use super::scenes_view_model::ScenesPaneViewModel;

#[injectable]
#[inject(
    |receiver: Receiver<SceneSelectedEvent>,
     sender: Sender<SelectedSceneChangedEvent>,
     selected_scene_changed_receiver: Receiver<SelectedSceneChangedEvent>,
     global_state: Rc<RefCell<GlobalStateModel>>,
     state_machine: Option<Rc<RefCell<ApplicationStateMachine>>>,
     scene_selected_sender: Sender<SceneSelectedEvent>| {
        Self::new(
            receiver,
            sender,
            selected_scene_changed_receiver,
            global_state,
            state_machine,
            scene_selected_sender,
        )
    }
)]
pub struct SelectSceneBehavior {
    receiver: Receiver<SceneSelectedEvent>,
    sender: Sender<SelectedSceneChangedEvent>,
    selected_scene_changed_receiver: Receiver<SelectedSceneChangedEvent>,
    global_state: Rc<RefCell<GlobalStateModel>>,
    state_machine: Option<Rc<RefCell<ApplicationStateMachine>>>,
    scene_selected_sender: Sender<SceneSelectedEvent>,
}

impl SelectSceneBehavior {
    pub fn new(
        receiver: Receiver<SceneSelectedEvent>,
        sender: Sender<SelectedSceneChangedEvent>,
        selected_scene_changed_receiver: Receiver<SelectedSceneChangedEvent>,
        global_state: Rc<RefCell<GlobalStateModel>>,
        state_machine: Option<Rc<RefCell<ApplicationStateMachine>>>,
        scene_selected_sender: Sender<SceneSelectedEvent>,
    ) -> Self {
        Self {
            receiver,
            sender,
            selected_scene_changed_receiver,
            global_state,
            state_machine,
            scene_selected_sender,
        }
    }

    fn try_handle(&self, view_model: &mut ScenesPaneViewModel) -> bool {
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

impl Behavior<ScenesPaneViewModel> for SelectSceneBehavior {
    fn activate(
        &self,
        view_model: &mut ScenesPaneViewModel,
        _disposables: &mut CompositeDisposable) {
        BehaviorLog::behavior_activated("SelectSceneBehavior", "ScenesPaneViewModel");

        let receiver = self.receiver.clone();
        let sender = self.sender.clone();
        let selected_scene_changed_receiver = self.selected_scene_changed_receiver.clone();
        let global_state = self.global_state.clone();
        let state_machine = self.state_machine.clone();
        let scene_selected_sender = self.scene_selected_sender.clone();

        view_model.set_select_scene_handler(Some(Rc::new(move |view_model, index| {
            let Some(selected_scene) = view_model.scenes.get(index).cloned() else {
                return;
            };

            let publisher = PublishSceneSelectedBehavior::new(scene_selected_sender.clone());
            publisher.publish(selected_scene);

            let selection = SelectSceneBehavior::new(
                receiver.clone(),
                sender.clone(),
                selected_scene_changed_receiver.clone(),
                global_state.clone(),
                state_machine.clone(),
                scene_selected_sender.clone(),
            );
            let _ = selection.try_handle(view_model);

            let mut apply_behavior = ApplyPlacementModeBehavior::new(
                global_state.clone(),
                state_machine.clone(),
                selected_scene_changed_receiver.clone(),
            );
            let _ = apply_behavior.try_handle();
        })));
    }
}
