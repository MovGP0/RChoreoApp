use std::sync::Once;

use nject::injectable;

use crate::states::ViewSceneState;
use crate::traits::{ApplicationState, ApplicationTrigger, GlobalStateModel};
use crate::transitions::{default_transitions, StateTransition};

#[injectable]
#[inject(|global_state: Box<dyn GlobalStateModel>, transitions: Vec<StateTransition>| Self {
    global_state,
    transitions,
    state: Box::new(ViewSceneState),
})]
pub struct ApplicationStateMachine {
    global_state: Box<dyn GlobalStateModel>,
    transitions: Vec<StateTransition>,
    state: Box<dyn ApplicationState>,
}

fn ensure_logger() {
    static LOGGER_INIT: Once = Once::new();

    LOGGER_INIT.call_once(|| {
        let _ = env_logger::builder().is_test(cfg!(test)).try_init();
    });
}

impl ApplicationStateMachine {
    pub fn new(global_state: Box<dyn GlobalStateModel>, transitions: Vec<StateTransition>) -> Self {
        Self {
            global_state,
            transitions,
            state: Box::new(ViewSceneState),
        }
    }

    pub fn with_default_transitions(global_state: Box<dyn GlobalStateModel>) -> Self {
        Self::new(global_state, default_transitions())
    }

    pub fn state(&self) -> &dyn ApplicationState {
        self.state.as_ref()
    }

    pub fn try_apply(&mut self, trigger: &dyn ApplicationTrigger) -> bool {
        let state_kind = self.state.kind();
        let trigger_kind = trigger.kind();

        for transition in &self.transitions {
            if !transition.from_state.is_assignable_from(state_kind)
                || !transition.trigger.is_assignable_from(trigger_kind)
            {
                continue;
            }

            if !transition.can_apply(self.global_state.as_ref(), self.state.as_ref(), trigger) {
                continue;
            }

            let next_state = (transition.apply)(self.global_state.as_ref(), self.state.as_ref(), trigger);
            let next_kind = next_state.kind();

            if next_kind != state_kind {
                ensure_logger();
                log::info!(
                    "ApplicationStateMachine transition: {:?} -> {:?} (trigger: {:?})",
                    state_kind,
                    next_kind,
                    trigger_kind
                );
            }

            self.state = next_state;
            return true;
        }

        false
    }
}
