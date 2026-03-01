use super::actions::SliderWithTicksAction;
use super::state::SliderWithTicksState;

pub fn reduce(state: &mut SliderWithTicksState, action: SliderWithTicksAction) {
    match action {
        SliderWithTicksAction::Initialize => {}
        SliderWithTicksAction::SetRange { minimum, maximum } => {
            state.minimum = minimum;
            state.maximum = maximum;
            state.value = state.value.clamp(state.minimum, state.maximum);
        }
        SliderWithTicksAction::SetValue { value } => {
            state.value = value.clamp(state.minimum, state.maximum);
        }
        SliderWithTicksAction::SetTickValues { tick_values } => {
            state.tick_values = tick_values;
        }
        SliderWithTicksAction::SetTickColor { tick_color } => {
            state.tick_color = tick_color;
        }
        SliderWithTicksAction::SetEnabled { is_enabled } => {
            state.is_enabled = is_enabled;
        }
    }
}
