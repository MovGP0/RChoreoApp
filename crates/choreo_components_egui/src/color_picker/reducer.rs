use super::actions::ColorPickerAction;
use super::state::ColorChangedEvent;
use super::state::ColorPickerState;
use super::state::Hsb;

#[must_use]
pub fn reduce(state: &mut ColorPickerState, action: ColorPickerAction) -> Option<ColorChangedEvent> {
    match action {
        ColorPickerAction::Initialize => None,
        ColorPickerAction::SetColor { color } => set_color(state, color),
        ColorPickerAction::SetHsb { hsb } => set_hsb(state, hsb),
        ColorPickerAction::UpdateFromSlider { brightness } => set_hsb(
            state,
            Hsb {
                brightness,
                ..state.hsb
            },
        ),
        ColorPickerAction::UpdateFromWheel { hue, saturation } => set_hsb(
            state,
            Hsb {
                hue,
                saturation,
                brightness: state.hsb.brightness,
            },
        ),
        ColorPickerAction::SetValueSliderPosition { position } => {
            state.value_slider_position = position;
            None
        }
    }
}

fn set_color(state: &mut ColorPickerState, color: egui::Color32) -> Option<ColorChangedEvent> {
    if color == state.selected_color {
        return None;
    }

    let old_color = state.selected_color;
    state.selected_color = color;
    state.hsb = Hsb::from_color(color);

    Some(ColorChangedEvent::new(old_color, color))
}

fn set_hsb(state: &mut ColorPickerState, hsb: Hsb) -> Option<ColorChangedEvent> {
    if hsb == state.hsb {
        return None;
    }

    let old_color = state.selected_color;
    let normalized = hsb.normalized();
    let new_color = normalized.to_color();

    state.hsb = normalized;
    state.selected_color = new_color;

    if old_color == new_color {
        return None;
    }

    Some(ColorChangedEvent::new(old_color, new_color))
}
