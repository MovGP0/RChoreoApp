use super::actions::ChoreographySettingsAction;
use super::state::ChoreographySettingsState;

pub fn reduce(state: &mut ChoreographySettingsState, action: ChoreographySettingsAction) {
    match action {
        ChoreographySettingsAction::Initialize => {}
        ChoreographySettingsAction::ToggleFlag { key } => {
            let previous = state.flags.get(&key).copied().unwrap_or(false);
            state.flags.insert(key, !previous);
        }
    }
}
