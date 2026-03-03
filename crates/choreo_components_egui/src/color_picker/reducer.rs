use super::actions::ColorPickerAction;
use super::state::ColorChangedEvent;
use super::state::ColorPickerState;
use super::state::Hsb;

#[must_use]
pub fn reduce(
    state: &mut ColorPickerState,
    action: ColorPickerAction,
) -> Option<ColorChangedEvent> {
    match action {
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
        ColorPickerAction::UpdateFromWheelPoint {
            x,
            y,
            center_x,
            center_y,
            radius_px,
        } => {
            let (hue, saturation) = map_wheel_point_to_hue_saturation(
                x,
                y,
                center_x,
                center_y,
                radius_px,
            );

            set_hsb(
                state,
                Hsb {
                    hue,
                    saturation,
                    brightness: state.hsb.brightness,
                },
            )
        }
        ColorPickerAction::SetValueSliderPosition { position } => {
            state.value_slider_position = position;
            None
        }
    }
}

fn map_wheel_point_to_hue_saturation(
    x: f32,
    y: f32,
    center_x: f32,
    center_y: f32,
    radius_px: f32,
) -> (f64, f64) {
    let dx = f64::from(x - center_x);
    let dy = f64::from(y - center_y);
    let distance = (dx * dx + dy * dy).sqrt();
    let radius = f64::from(radius_px).max(0.0);
    let clamped_distance = distance.min(radius);
    let mut hue = dy.atan2(dx).to_degrees();
    if hue < 0.0 {
        hue += 360.0;
    }

    let saturation = if radius <= f64::EPSILON {
        0.0
    } else {
        clamped_distance / radius
    };

    (hue, saturation)
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
