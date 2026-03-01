use super::actions::AudioPlayerAction;
use super::state::AudioPlayerState;

pub fn reduce(state: &mut AudioPlayerState, action: AudioPlayerAction) {
    match action {
        AudioPlayerAction::Initialize => {}
        AudioPlayerAction::ToggleFlag { key } => {
            let previous = state.flags.get(&key).copied().unwrap_or(false);
            state.flags.insert(key, !previous);
        }
    }
}
