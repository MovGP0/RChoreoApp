use super::actions::DancersAction;
use super::state::DancersState;

pub fn reduce(state: &mut DancersState, action: DancersAction) {
    match action {
        DancersAction::Initialize => {}
        DancersAction::ToggleFlag { key } => {
            let previous = state.flags.get(&key).copied().unwrap_or(false);
            state.flags.insert(key, !previous);
        }
    }
}
