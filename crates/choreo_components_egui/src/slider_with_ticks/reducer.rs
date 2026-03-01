use super::actions::SliderWithTicksAction;
use super::state::SliderWithTicksState;

pub fn reduce(state: &mut SliderWithTicksState, action: SliderWithTicksAction) {
    match action {
        SliderWithTicksAction::Initialize => {}
        SliderWithTicksAction::ToggleFlag { key } => {
            let previous = state.flags.get(&key).copied().unwrap_or(false);
            state.flags.insert(key, !previous);
        }
    }
}
