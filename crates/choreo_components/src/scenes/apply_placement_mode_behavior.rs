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
use crate::observability::start_internal_span;

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
        let mut span = start_internal_span("scenes.apply_placement_mode", None);
        span.set_bool_attribute("choreo.scenes.has_selected_scene", selected_scene.is_some());
        span.set_bool_attribute("choreo.scenes.has_state_machine", state_machine.is_some());
        if selected_scene.is_none() {
            let updated = global_state.try_update(|global_state| {
                global_state.is_place_mode = false;
            });
            if !updated {
                span.set_bool_attribute("choreo.success", false);
                return;
            }
            span.set_bool_attribute("choreo.success", true);
            span.set_bool_attribute("choreo.scenes.is_place_mode", false);
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
            span.set_bool_attribute("choreo.success", false);
            return;
        };
        span.set_f64_attribute("choreo.scenes.dancer_count", dancer_count as f64);
        span.set_f64_attribute("choreo.scenes.position_count", position_count as f64);
        span.set_string_attribute("choreo.scene.id", format!("{scene_id:?}"));

        let should_place = dancer_count > 0 && position_count < dancer_count;
        span.set_bool_attribute("choreo.scenes.should_place", should_place);
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
            span.set_bool_attribute("choreo.success", false);
            return;
        }
        span.set_bool_attribute("choreo.success", true);
        span.set_bool_attribute("choreo.scenes.is_place_mode", should_place);

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
