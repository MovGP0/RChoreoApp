use super::actions::PreferencesAction;
use super::state::PreferenceWriteIntent;
use super::state::PreferencesState;

pub fn reduce(state: &mut PreferencesState, action: PreferencesAction) {
    match action {
        PreferencesAction::Initialize { app_name } => {
            state.app_name = app_name;
        }
        PreferencesAction::LoadString { key, value } => {
            state.strings.insert(key, value);
        }
        PreferencesAction::LoadBool { key, value } => {
            state.bools.insert(key, value);
        }
        PreferencesAction::SetString { key, value } => {
            state.strings.insert(key.clone(), value.clone());
            state
                .pending_writes
                .push(PreferenceWriteIntent::SetString { key, value });
        }
        PreferencesAction::SetBool { key, value } => {
            state.bools.insert(key.clone(), value);
            state
                .pending_writes
                .push(PreferenceWriteIntent::SetBool { key, value });
        }
        PreferencesAction::ToggleBool { key, default } => {
            let value = !state.get_bool(&key, default);
            state.bools.insert(key.clone(), value);
            state
                .pending_writes
                .push(PreferenceWriteIntent::SetBool { key, value });
        }
        PreferencesAction::Remove { key } => {
            state.strings.remove(&key);
            state.bools.remove(&key);
            state
                .pending_writes
                .push(PreferenceWriteIntent::Remove { key });
        }
        PreferencesAction::ClearPendingWrites => {
            state.pending_writes.clear();
        }
        PreferencesAction::ClearAll => {
            state.strings.clear();
            state.bools.clear();
            state.pending_writes.clear();
        }
    }
}
