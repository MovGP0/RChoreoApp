use super::actions::TimeAction;
use super::state::{TimeState, format_seconds, parse_timestamp_seconds, system_now_unix_nanos};

pub fn reduce(state: &mut TimeState, action: TimeAction) {
    match action {
        TimeAction::InitializeCurrentTime => {
            state.last_now_unix_nanos = system_now_unix_nanos();
        }
        TimeAction::SetTimestampInput { value } => {
            state.timestamp_input = value;
        }
        TimeAction::ParseTimestampInput => {
            state.parsed_seconds = parse_timestamp_seconds(&state.timestamp_input);
            if let Some(seconds) = state.parsed_seconds {
                state.formatted_seconds = format_seconds(seconds);
            }
        }
        TimeAction::SetSeconds { value } => {
            state.parsed_seconds = Some(value);
            state.formatted_seconds = format_seconds(value);
        }
    }
}
