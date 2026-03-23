use crate::behavior::Behavior;
use crate::behavior::CompositeDisposable;

use super::actions::DancersAction;
use super::reducer;
use super::state::DancersState;

#[derive(Default)]
pub struct DancersProvider {
    state: DancersState,
    disposables: CompositeDisposable,
}

impl DancersProvider {
    #[must_use]
    pub fn new(behaviors: Vec<Box<dyn Behavior<DancersState>>>) -> Self {
        let mut state = DancersState::default();
        let mut disposables = CompositeDisposable::new();
        for behavior in behaviors {
            behavior.activate(&mut state, &mut disposables);
        }

        Self { state, disposables }
    }

    #[must_use]
    pub fn state(&self) -> &DancersState {
        &self.state
    }

    pub fn state_mut(&mut self) -> &mut DancersState {
        &mut self.state
    }

    pub fn dispatch(&mut self, action: DancersAction) {
        reducer::reduce(&mut self.state, action);
    }

    pub fn reload(&mut self) {
        self.dispatch(DancersAction::ReloadFromGlobal);
    }
}

impl Drop for DancersProvider {
    fn drop(&mut self) {
        self.disposables.dispose_all();
    }
}
