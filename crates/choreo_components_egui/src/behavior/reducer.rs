use super::actions::BehaviorAction;
use super::state::BehaviorState;

pub fn reduce(state: &mut BehaviorState, action: BehaviorAction) {
    match action {
        BehaviorAction::Initialize => state.dispose_all(),
        BehaviorAction::AddDisposable { disposable } => state.add_disposable(disposable),
        BehaviorAction::DisposeAll => state.dispose_all(),
    }
}
