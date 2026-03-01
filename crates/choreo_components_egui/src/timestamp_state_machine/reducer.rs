use super::actions::TimestampStateMachineAction;
use super::state::TimestampStateMachineState;

pub fn reduce(state: &mut TimestampStateMachineState, action: TimestampStateMachineAction) {
    match action {
        TimestampStateMachineAction::Initialize => {}
        TimestampStateMachineAction::ToggleFlag { key } => {
            let previous = state.flags.get(&key).copied().unwrap_or(false);
            state.flags.insert(key, !previous);
        }
    }
}
