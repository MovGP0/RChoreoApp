use super::actions::ColorPickerAction;
use super::state::ColorPickerState;

pub fn reduce(state: &mut ColorPickerState, action: ColorPickerAction) {
    match action {
        ColorPickerAction::Initialize => {}
        ColorPickerAction::ToggleFlag { key } => {
            let previous = state.flags.get(&key).copied().unwrap_or(false);
            state.flags.insert(key, !previous);
        }
    }
}
