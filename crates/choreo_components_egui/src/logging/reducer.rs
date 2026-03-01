use super::actions::LoggingAction;
use super::state::{LogEntry, LogLevel, LoggingState};
use super::types::activation_message;

pub fn reduce(state: &mut LoggingState, action: LoggingAction) {
    match action {
        LoggingAction::Initialize => state.entries.clear(),
        LoggingAction::BehaviorActivated { name, view_model } => push_entry(
            state,
            LogLevel::Debug,
            activation_message(&name, &view_model),
        ),
        LoggingAction::RecordDebug { message } => push_entry(state, LogLevel::Debug, message),
        LoggingAction::RecordInfo { message } => push_entry(state, LogLevel::Info, message),
        LoggingAction::RecordWarn { message } => push_entry(state, LogLevel::Warn, message),
        LoggingAction::RecordError { message } => push_entry(state, LogLevel::Error, message),
        LoggingAction::SetMaxEntries { max_entries } => {
            state.max_entries = max_entries.max(1);
            truncate_to_max(state);
        }
        LoggingAction::ClearEntries => state.entries.clear(),
    }
}

fn push_entry(state: &mut LoggingState, level: LogLevel, message: String) {
    state.entries.push(LogEntry { level, message });
    truncate_to_max(state);
}

fn truncate_to_max(state: &mut LoggingState) {
    if state.entries.len() <= state.max_entries {
        return;
    }
    let overflow = state.entries.len() - state.max_entries;
    state.entries.drain(0..overflow);
    state.dropped_entries += overflow as u64;
}
