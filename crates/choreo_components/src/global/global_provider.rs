use std::cell::RefCell;
use std::rc::Rc;

use choreo_state_machine::ApplicationStateMachine;

use super::GlobalStateModel;

pub struct GlobalProvider {
    global_state: Rc<RefCell<GlobalStateModel>>,
    state_machine: Rc<RefCell<ApplicationStateMachine>>,
}

impl GlobalProvider {
    pub fn new() -> Self
    {
        let global_state = Rc::new(RefCell::new(GlobalStateModel::default()));
        let state_machine = Rc::new(RefCell::new(
            ApplicationStateMachine::with_default_transitions(Box::new(
                GlobalStateModel::default(),
            )),
        ));

        Self {
            global_state,
            state_machine,
        }
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
