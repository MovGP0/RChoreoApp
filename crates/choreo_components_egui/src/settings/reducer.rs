use super::actions::SettingsAction;
use super::state::SettingsState;

pub fn reduce(state: &mut SettingsState, action: SettingsAction) {
    match action {
        SettingsAction::Initialize => {}
        SettingsAction::ToggleFlag { key } => {
            let previous = state.flags.get(&key).copied().unwrap_or(false);
            state.flags.insert(key, !previous);
        }
    }
}
