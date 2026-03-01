use super::actions::TimeAction;
use super::state::TimeState;

pub fn reduce(state: &mut TimeState, action: TimeAction) {
    match action {
        TimeAction::Initialize => {}
        TimeAction::ToggleFlag { key } => {
            let previous = state.flags.get(&key).copied().unwrap_or(false);
            state.flags.insert(key, !previous);
        }
    }
}
