use std::cell::RefCell;
use std::rc::Rc;

use choreo_state_machine::ApplicationStateMachine;
use choreo_state_machine::MovePositionsCompletedTrigger;
use choreo_state_machine::MovePositionsStartedTrigger;
use choreo_state_machine::RotateAroundCenterCompletedTrigger;
use choreo_state_machine::RotateAroundCenterSelectionCompletedTrigger;
use choreo_state_machine::RotateAroundCenterStartedTrigger;
use choreo_state_machine::ScaleAroundDancerCompletedTrigger;
use choreo_state_machine::ScaleAroundDancerSelectionCompletedTrigger;
use choreo_state_machine::ScaleAroundDancerStartedTrigger;
use choreo_state_machine::ScalePositionsCompletedTrigger;
use choreo_state_machine::ScalePositionsSelectionCompletedTrigger;
use choreo_state_machine::ScalePositionsStartedTrigger;
use nject::injectable;

use crate::behavior::Behavior;
use crate::behavior::CompositeDisposable;
use crate::global::GlobalStateActor;
use crate::global::InteractionMode;
use crate::logging::BehaviorLog;

use super::main_view_model::MainViewModel;

#[injectable]
#[inject(
    |global_state_store: Rc<GlobalStateActor>, state_machine: Rc<RefCell<ApplicationStateMachine>>| {
        Self::new(global_state_store, state_machine)
    }
)]
#[derive(Clone)]
pub struct ApplyInteractionModeBehavior {
    global_state_store: Rc<GlobalStateActor>,
    state_machine: Rc<RefCell<ApplicationStateMachine>>,
}

impl ApplyInteractionModeBehavior {
    pub fn new(
        global_state_store: Rc<GlobalStateActor>,
        state_machine: Rc<RefCell<ApplicationStateMachine>>,
    ) -> Self {
        Self {
            global_state_store,
            state_machine,
        }
    }

    pub fn apply(&self, mode: InteractionMode) {
        let Some(selected_positions_count) = self
            .global_state_store
            .try_with_state(|state| state.selected_positions.len())
        else {
            return;
        };

        let mut state_machine = self.state_machine.borrow_mut();
        match mode {
            InteractionMode::Move => {
                state_machine.try_apply(&RotateAroundCenterCompletedTrigger);
                state_machine.try_apply(&ScalePositionsCompletedTrigger);
                state_machine.try_apply(&ScaleAroundDancerCompletedTrigger);
                state_machine.try_apply(&MovePositionsStartedTrigger);
            }
            InteractionMode::RotateAroundCenter if selected_positions_count == 0 => {
                state_machine.try_apply(&ScalePositionsCompletedTrigger);
                state_machine.try_apply(&ScaleAroundDancerCompletedTrigger);
                state_machine.try_apply(&RotateAroundCenterStartedTrigger);
            }
            InteractionMode::RotateAroundCenter => {
                state_machine.try_apply(&ScalePositionsCompletedTrigger);
                state_machine.try_apply(&ScaleAroundDancerCompletedTrigger);
                state_machine.try_apply(&RotateAroundCenterStartedTrigger);
                state_machine.try_apply(&RotateAroundCenterSelectionCompletedTrigger);
            }
            InteractionMode::RotateAroundDancer if selected_positions_count == 0 => {
                state_machine.try_apply(&RotateAroundCenterCompletedTrigger);
                state_machine.try_apply(&ScalePositionsCompletedTrigger);
                state_machine.try_apply(&ScaleAroundDancerStartedTrigger);
            }
            InteractionMode::RotateAroundDancer => {
                state_machine.try_apply(&RotateAroundCenterCompletedTrigger);
                state_machine.try_apply(&ScalePositionsCompletedTrigger);
                state_machine.try_apply(&ScaleAroundDancerStartedTrigger);
                state_machine.try_apply(&ScaleAroundDancerSelectionCompletedTrigger);
            }
            InteractionMode::Scale if selected_positions_count == 0 => {
                state_machine.try_apply(&RotateAroundCenterCompletedTrigger);
                state_machine.try_apply(&ScaleAroundDancerCompletedTrigger);
                state_machine.try_apply(&ScalePositionsStartedTrigger);
            }
            InteractionMode::Scale => {
                state_machine.try_apply(&RotateAroundCenterCompletedTrigger);
                state_machine.try_apply(&ScaleAroundDancerCompletedTrigger);
                state_machine.try_apply(&ScalePositionsStartedTrigger);
                state_machine.try_apply(&ScalePositionsSelectionCompletedTrigger);
            }
            InteractionMode::View | InteractionMode::LineOfSight => {
                state_machine.try_apply(&MovePositionsCompletedTrigger);
                state_machine.try_apply(&RotateAroundCenterCompletedTrigger);
                state_machine.try_apply(&ScalePositionsCompletedTrigger);
                state_machine.try_apply(&ScaleAroundDancerCompletedTrigger);
            }
        }
    }
}

impl Behavior<MainViewModel> for ApplyInteractionModeBehavior {
    fn activate(&self, _view_model: &mut MainViewModel, _disposables: &mut CompositeDisposable) {
        BehaviorLog::behavior_activated("ApplyInteractionModeBehavior", "MainViewModel");
    }
}
