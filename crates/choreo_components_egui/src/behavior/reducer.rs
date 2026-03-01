use super::actions::BehaviorAction;
use super::state::BehaviorState;

pub fn reduce(state: &mut BehaviorState, action: BehaviorAction) {
    match action {
        BehaviorAction::Initialize => {}
        BehaviorAction::ToggleFlag { key } => {
            let previous = state.flags.get(&key).copied().unwrap_or(false);
            state.flags.insert(key, !previous);
        }
    }
}
