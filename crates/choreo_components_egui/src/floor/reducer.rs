use super::actions::FloorAction;
use super::state::FloorState;

pub fn reduce(state: &mut FloorState, action: FloorAction) {
    match action {
        FloorAction::Initialize => {}
        FloorAction::ToggleFlag { key } => {
            let previous = state.flags.get(&key).copied().unwrap_or(false);
            state.flags.insert(key, !previous);
        }
    }
}
