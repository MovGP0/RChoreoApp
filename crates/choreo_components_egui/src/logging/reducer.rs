use super::actions::LoggingAction;
use super::state::LoggingState;

pub fn reduce(state: &mut LoggingState, action: LoggingAction) {
    match action {
        LoggingAction::Initialize => {}
        LoggingAction::ToggleFlag { key } => {
            let previous = state.flags.get(&key).copied().unwrap_or(false);
            state.flags.insert(key, !previous);
        }
    }
}
