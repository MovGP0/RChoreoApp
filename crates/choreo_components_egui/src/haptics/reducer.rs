use super::actions::HapticsAction;
use super::state::HapticsState;

pub fn reduce(state: &mut HapticsState, action: HapticsAction) {
    match action {
        HapticsAction::Initialize => {}
        HapticsAction::ToggleFlag { key } => {
            let previous = state.flags.get(&key).copied().unwrap_or(false);
            state.flags.insert(key, !previous);
        }
    }
}
