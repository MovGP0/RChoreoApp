use super::actions::PreferencesAction;
use super::state::PreferencesState;

pub fn reduce(state: &mut PreferencesState, action: PreferencesAction) {
    match action {
        PreferencesAction::Initialize => {}
        PreferencesAction::ToggleFlag { key } => {
            let previous = state.flags.get(&key).copied().unwrap_or(false);
            state.flags.insert(key, !previous);
        }
    }
}
