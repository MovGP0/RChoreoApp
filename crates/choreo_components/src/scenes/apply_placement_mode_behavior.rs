use std::cell::RefCell;
use std::rc::Rc;

use crossbeam_channel::Receiver;
use choreo_state_machine::{
    ApplicationStateMachine, ApplicationTrigger, PlacePositionsCompletedTrigger,
    PlacePositionsStartedTrigger,
};
use nject::injectable;

use crate::global::GlobalStateModel;
use crate::behavior::{Behavior, CompositeDisposable};
use crate::logging::BehaviorLog;

use super::messages::SelectedSceneChangedEvent;
use super::scenes_view_model::SceneViewModel;

#[injectable]
#[inject(
    |global_state: Rc<RefCell<GlobalStateModel>>,
     state_machine: Option<Rc<RefCell<ApplicationStateMachine>>>,
     receiver: Receiver<SelectedSceneChangedEvent>| {
        Self::new(global_state, state_machine, receiver)
    }
)]
pub struct ApplyPlacementModeBehavior {
    global_state: Rc<RefCell<GlobalStateModel>>,
    state_machine: Option<Rc<RefCell<ApplicationStateMachine>>>,
    receiver: Receiver<SelectedSceneChangedEvent>,
}

impl ApplyPlacementModeBehavior {
    pub fn new(
        global_state: Rc<RefCell<GlobalStateModel>>,
        state_machine: Option<Rc<RefCell<ApplicationStateMachine>>>,
        receiver: Receiver<SelectedSceneChangedEvent>,
    ) -> Self {
        Self {
            global_state,
            state_machine,
            receiver,
        }
    }

    pub fn try_handle(&mut self) -> bool {
        match self.receiver.try_recv() {
            Ok(event) => {
                self.apply_for_scene(event.selected_scene.as_ref());
                true
            }
            Err(_) => false,
        }
    }

    fn apply_for_scene(&self, selected_scene: Option<&SceneViewModel>) {
        if selected_scene.is_none() {
            let mut global_state = self.global_state.borrow_mut();
            global_state.is_place_mode = false;
            if let Some(state_machine) = &self.state_machine {
                state_machine
                    .borrow_mut()
                    .try_apply(&PlacePositionsCompletedTrigger);
            }
            return;
        }

        let selected_scene = selected_scene.unwrap();
        let (dancer_count, position_count, scene_id) = {
            let global_state = self.global_state.borrow();
            let choreography = &global_state.choreography;
            let position_count = choreography
                .scenes
                .iter()
                .find(|scene| scene.scene_id == selected_scene.scene_id)
                .map(|scene| scene.positions.len())
                .unwrap_or_default();
            (choreography.dancers.len(), position_count, selected_scene.scene_id)
        };

        let should_place = dancer_count > 0 && position_count < dancer_count;
        let mut global_state = self.global_state.borrow_mut();
        global_state.is_place_mode = should_place;
        if let Some(state_machine) = &self.state_machine {
            let trigger: &dyn ApplicationTrigger = if should_place {
                &PlacePositionsStartedTrigger
            } else {
                &PlacePositionsCompletedTrigger
            };
            state_machine.borrow_mut().try_apply(trigger);
        }

        if should_place
            && let Some(scene) = global_state
                .choreography
                .scenes
                .iter_mut()
                .find(|scene| scene.scene_id == scene_id)
        {
            for position in &mut scene.positions {
                position.dancer = None;
            }
        }
    }
}

impl Behavior<super::scenes_view_model::ScenesPaneViewModel> for ApplyPlacementModeBehavior {
    fn initialize(
        &self,
        _view_model: &mut super::scenes_view_model::ScenesPaneViewModel,
        _disposables: &mut CompositeDisposable,
    ) {
        BehaviorLog::behavior_activated("ApplyPlacementModeBehavior", "ScenesPaneViewModel");
    }
}
