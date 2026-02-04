use std::cell::RefCell;
use std::rc::Rc;

use choreo_state_machine::ApplicationStateMachine;

use super::{GlobalStateModel, GlobalStateActor};

pub struct GlobalProvider {
    global_state_store: Rc<GlobalStateActor>,
    global_state: Rc<RefCell<GlobalStateModel>>,
    state_machine: Rc<RefCell<ApplicationStateMachine>>,
}

impl GlobalProvider {
    pub fn new() -> Self
    {
        let global_state_store = GlobalStateActor::new();
        let global_state = global_state_store.state_handle();
        let state_machine = Rc::new(RefCell::new(
            ApplicationStateMachine::with_default_transitions(Box::new(
                GlobalStateModel::default(),
            )),
        ));

        Self {
            global_state_store,
            global_state,
            state_machine,
        }
    }

    pub fn global_state_store(&self) -> Rc<GlobalStateActor>
    {
        Rc::clone(&self.global_state_store)
    }

    pub fn global_state(&self) -> Rc<RefCell<GlobalStateModel>> {
        Rc::clone(&self.global_state)
    }

    pub fn state_machine(&self) -> Rc<RefCell<ApplicationStateMachine>> {
        Rc::clone(&self.state_machine)
    }
}

impl Default for GlobalProvider {
    fn default() -> Self
    {
        Self::new()
    }
}
