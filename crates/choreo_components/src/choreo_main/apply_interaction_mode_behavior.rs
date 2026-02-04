use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use crossbeam_channel::Receiver;
use choreo_state_machine::{
    ApplicationStateMachine, MovePositionsCompletedTrigger, MovePositionsStartedTrigger,
    RotateAroundCenterCompletedTrigger, RotateAroundCenterSelectionCompletedTrigger,
    RotateAroundCenterStartedTrigger, ScaleAroundDancerCompletedTrigger,
    ScaleAroundDancerSelectionCompletedTrigger, ScaleAroundDancerStartedTrigger,
    ScalePositionsCompletedTrigger, ScalePositionsSelectionCompletedTrigger,
    ScalePositionsStartedTrigger,
};
use nject::injectable;
use slint::TimerMode;

use crate::behavior::{Behavior, CompositeDisposable};
use crate::behavior::TimerDisposable;
use crate::global::{GlobalStateActor, InteractionMode};
use crate::logging::BehaviorLog;

use super::main_view_model::MainViewModel;

#[derive(Clone)]
#[injectable]
#[inject(
    |global_state: Rc<GlobalStateActor>,
     state_machine: Rc<RefCell<ApplicationStateMachine>>,
     receiver: Receiver<InteractionMode>| {
        Self::new(global_state, state_machine, receiver)
    }
)]
pub struct ApplyInteractionModeBehavior {
    global_state: Rc<GlobalStateActor>,
    state_machine: Rc<RefCell<ApplicationStateMachine>>,
    receiver: Receiver<InteractionMode>,
}

impl ApplyInteractionModeBehavior {
    pub fn new(
        global_state: Rc<GlobalStateActor>,
        state_machine: Rc<RefCell<ApplicationStateMachine>>,
        receiver: Receiver<InteractionMode>,
    ) -> Self {
        Self {
            global_state,
            state_machine,
            receiver,
        }
    }

    fn apply_mode(&self, mode: InteractionMode) {
        let Some(selected_positions) = self.global_state.try_with_state(|global_state| {
            global_state.selected_positions.len()
        }) else {
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

impl Behavior<MainViewModel> for ApplyInteractionModeBehavior {
    fn activate(&self, _view_model: &mut MainViewModel, disposables: &mut CompositeDisposable) {
        BehaviorLog::behavior_activated("ApplyInteractionModeBehavior", "MainViewModel");
        let receiver = self.receiver.clone();
        let behavior = self.clone();
        let timer = slint::Timer::default();
        timer.start(TimerMode::Repeated, Duration::from_millis(16), move || {
            while let Ok(mode) = receiver.try_recv() {
                behavior.apply_mode(mode);
            }
        });
        disposables.add(Box::new(TimerDisposable::new(timer)));
    }
}
