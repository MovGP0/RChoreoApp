use super::actions::ShellAction;
use super::state::ShellState;

pub fn reduce(state: &mut ShellState, action: ShellAction) {
    match action {
        ShellAction::Initialize => {}
        ShellAction::ToggleFlag { key } => {
            let previous = state.flags.get(&key).copied().unwrap_or(false);
            state.flags.insert(key, !previous);
        }
    }
}
