use std::cell::RefCell;
use std::rc::Rc;

use choreo_state_machine::{
    ApplicationStateMachine, ApplicationTrigger, PlacePositionsCompletedTrigger,
    PlacePositionsStartedTrigger,
};
use crossbeam_channel::Receiver;
use nject::injectable;
use slint::TimerMode;

use crate::behavior::{Behavior, CompositeDisposable, TimerDisposable};
use crate::global::GlobalStateActor;
use crate::logging::BehaviorLog;

use super::messages::SelectedSceneChangedEvent;
use super::scenes_view_model::SceneViewModel;

#[injectable]
#[inject(
    |global_state: Rc<GlobalStateActor>,
     state_machine: Option<Rc<RefCell<ApplicationStateMachine>>>,
     receiver: Receiver<SelectedSceneChangedEvent>| {
        Self::new(global_state, state_machine, receiver)
    }
)]
pub struct ApplyPlacementModeBehavior {
    global_state: Rc<GlobalStateActor>,
    state_machine: Option<Rc<RefCell<ApplicationStateMachine>>>,
    receiver: Receiver<SelectedSceneChangedEvent>,
}

impl ApplyPlacementModeBehavior {
    pub fn new(
        global_state: Rc<GlobalStateActor>,
        state_machine: Option<Rc<RefCell<ApplicationStateMachine>>>,
        receiver: Receiver<SelectedSceneChangedEvent>,
    ) -> Self {
        Self {
            global_state,
            state_machine,
            receiver,
        }
    }

    fn apply_for_scene(
        global_state: &Rc<GlobalStateActor>,
        state_machine: &Option<Rc<RefCell<ApplicationStateMachine>>>,
        selected_scene: Option<&SceneViewModel>,
    ) {
        if selected_scene.is_none() {
            let updated = global_state.try_update(|global_state| {
                global_state.is_place_mode = false;
            });
            if !updated {
                return;
            }
            if let Some(state_machine) = state_machine {
                state_machine
                    .borrow_mut()
                    .try_apply(&PlacePositionsCompletedTrigger);
            }
            return;
        }

        let selected_scene = selected_scene.unwrap();
        let Some((dancer_count, position_count, scene_id)) =
            global_state.try_with_state(|global_state| {
                let choreography = &global_state.choreography;
                let position_count = choreography
                    .scenes
                    .iter()
                    .find(|scene| scene.scene_id == selected_scene.scene_id)
                    .map(|scene| scene.positions.len())
                    .unwrap_or_default();
                (
                    choreography.dancers.len(),
                    position_count,
                    selected_scene.scene_id,
                )
            })
        else {
            return;
        };

        let should_place = dancer_count > 0 && position_count < dancer_count;
        let updated = global_state.try_update(|global_state| {
            global_state.is_place_mode = should_place;
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
        });
        if !updated {
            return;
        }

        if let Some(state_machine) = state_machine {
            let trigger: &dyn ApplicationTrigger = if should_place {
                &PlacePositionsStartedTrigger
            } else {
                &PlacePositionsCompletedTrigger
            };
            state_machine.borrow_mut().try_apply(trigger);
        }
    }
}

impl Behavior<super::scenes_view_model::ScenesPaneViewModel> for ApplyPlacementModeBehavior {
    fn activate(
        &self,
        _view_model: &mut super::scenes_view_model::ScenesPaneViewModel,
        disposables: &mut CompositeDisposable,
    ) {
        BehaviorLog::behavior_activated("ApplyPlacementModeBehavior", "ScenesPaneViewModel");
        let receiver = self.receiver.clone();
        let global_state = self.global_state.clone();
        let state_machine = self.state_machine.clone();
        let selected_scene = global_state
            .try_with_state(|global_state| global_state.selected_scene.clone())
            .flatten();
        ApplyPlacementModeBehavior::apply_for_scene(
            &global_state,
            &state_machine,
            selected_scene.as_ref(),
        );
        let timer = slint::Timer::default();
        timer.start(
            TimerMode::Repeated,
            std::time::Duration::from_millis(16),
            move || {
                while let Ok(event) = receiver.try_recv() {
                    ApplyPlacementModeBehavior::apply_for_scene(
                        &global_state,
                        &state_machine,
                        event.selected_scene.as_ref(),
                    );
                }
            },
        );
        disposables.add(Box::new(TimerDisposable::new(timer)));
    }
}
