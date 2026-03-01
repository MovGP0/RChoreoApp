use super::actions::GlobalAction;
use super::state::GlobalState;

pub fn reduce(state: &mut GlobalState, action: GlobalAction) {
    match action {
        GlobalAction::Initialize => {}
        GlobalAction::ToggleFlag { key } => {
            let previous = state.flags.get(&key).copied().unwrap_or(false);
            state.flags.insert(key, !previous);
        }
    }
}
