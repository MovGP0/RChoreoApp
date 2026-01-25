use std::cell::RefCell;
use std::rc::Rc;

use choreo_state_machine::{
    ApplicationStateMachine, MovePositionsCompletedTrigger, MovePositionsStartedTrigger,
    RotateAroundCenterCompletedTrigger, RotateAroundCenterSelectionCompletedTrigger,
    RotateAroundCenterStartedTrigger, ScaleAroundDancerCompletedTrigger,
    ScaleAroundDancerSelectionCompletedTrigger, ScaleAroundDancerStartedTrigger,
    ScalePositionsCompletedTrigger, ScalePositionsSelectionCompletedTrigger,
    ScalePositionsStartedTrigger,
};
use nject::injectable;

use crate::global::{GlobalStateModel, InteractionMode};

#[injectable]
#[inject(
    |global_state: Rc<RefCell<GlobalStateModel>>, state_machine: Rc<RefCell<ApplicationStateMachine>>| {
        Self::new(global_state, state_machine)
    }
)]
pub struct ApplyInteractionModeBehavior {
    global_state: Rc<RefCell<GlobalStateModel>>,
    state_machine: Rc<RefCell<ApplicationStateMachine>>,
}

impl ApplyInteractionModeBehavior {
    pub fn new(
        global_state: Rc<RefCell<GlobalStateModel>>,
        state_machine: Rc<RefCell<ApplicationStateMachine>>,
    ) -> Self {
        Self {
            global_state,
            state_machine,
        }
    }

    pub fn apply_mode(&self, mode: InteractionMode) {
        let selected_positions = self.global_state.borrow().selected_positions.len();
        let mut state_machine = self.state_machine.borrow_mut();

        match mode {
            InteractionMode::Move => {
                state_machine.try_apply(&RotateAroundCenterCompletedTrigger);
                state_machine.try_apply(&ScalePositionsCompletedTrigger);
                state_machine.try_apply(&ScaleAroundDancerCompletedTrigger);
                state_machine.try_apply(&MovePositionsStartedTrigger);
            }
            InteractionMode::RotateAroundCenter if selected_positions == 0 => {
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
            InteractionMode::RotateAroundDancer if selected_positions == 0 => {
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
            InteractionMode::Scale if selected_positions == 0 => {
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
            _ => {
                state_machine.try_apply(&MovePositionsCompletedTrigger);
                state_machine.try_apply(&RotateAroundCenterCompletedTrigger);
                state_machine.try_apply(&ScalePositionsCompletedTrigger);
                state_machine.try_apply(&ScaleAroundDancerCompletedTrigger);
            }
        }
    }
}
