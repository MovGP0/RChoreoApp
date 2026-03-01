use super::actions::ObservabilityAction;
use super::state::ObservabilityState;

pub fn reduce(state: &mut ObservabilityState, action: ObservabilityAction) {
    match action {
        ObservabilityAction::Initialize => {}
        ObservabilityAction::ToggleFlag { key } => {
            let previous = state.flags.get(&key).copied().unwrap_or(false);
            state.flags.insert(key, !previous);
        }
    }
}
