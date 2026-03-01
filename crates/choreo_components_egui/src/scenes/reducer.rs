use super::actions::ScenesAction;
use super::state::ScenesState;

pub fn reduce(state: &mut ScenesState, action: ScenesAction) {
    match action {
        ScenesAction::Initialize => {}
        ScenesAction::ToggleFlag { key } => {
            let previous = state.flags.get(&key).copied().unwrap_or(false);
            state.flags.insert(key, !previous);
        }
    }
}
