#[path = "../../src/color_picker/actions.rs"]
mod actions;
#[path = "../../src/color_picker/reducer.rs"]
mod reducer;
#[path = "../../src/color_picker/state.rs"]
mod state;

use egui::Color32;

use actions::ColorPickerAction;
use reducer::reduce;
use state::ColorPickerDock;
use state::ColorPickerState;
use state::Hsb;

#[test]
fn color_picker_defaults_match_source_component() {
    let state = ColorPickerState::new();

    assert_eq!(state.selected_color, Color32::BLACK);
    assert_eq!(state.hsb, Hsb::new(0.0, 0.0, 0.0));
    assert_eq!(state.wheel_minimum_width, 160.0);
    assert_eq!(state.wheel_minimum_height, 160.0);
    assert_eq!(state.value_slider_position, ColorPickerDock::Bottom);
    assert_eq!(state.selection_thumb_size, 18.0);
    assert_eq!(state.selection_stroke_thickness, 2.0);
    assert_eq!(state.selection_stroke_color, Color32::WHITE);
}

#[test]
fn set_color_updates_hsb_and_emits_event() {
    let mut state = ColorPickerState::new();

    let event = reduce(
        &mut state,
        ColorPickerAction::SetColor {
            color: Color32::from_rgb(255, 0, 0),
        },
    );

    let event = event.expect("expected color changed event");
    assert_eq!(event.old_color, Color32::BLACK);
    assert_eq!(event.new_color, Color32::from_rgb(255, 0, 0));
    assert_eq!(state.selected_color, Color32::from_rgb(255, 0, 0));
    assert_eq!(state.hsb, Hsb::new(0.0, 1.0, 1.0));
}

#[test]
fn set_hsb_normalizes_values_and_updates_color() {
    let mut state = ColorPickerState::new();

    let event = reduce(
        &mut state,
        ColorPickerAction::SetHsb {
            hsb: Hsb::new(-30.0, 2.0, -1.0),
        },
    );

    assert!(event.is_none());
    assert_eq!(state.hsb, Hsb::new(330.0, 1.0, 0.0));
    assert_eq!(state.selected_color, Color32::BLACK);
}

#[test]
fn update_from_wheel_preserves_brightness() {
    let mut state = ColorPickerState::new();
    state.hsb = Hsb::new(0.0, 0.0, 0.25);
    state.selected_color = state.hsb.to_color();

    let _ = reduce(
        &mut state,
        ColorPickerAction::UpdateFromWheel {
            hue: 120.0,
            saturation: 1.0,
        },
    );

    assert_eq!(state.hsb, Hsb::new(120.0, 1.0, 0.25));
}

#[test]
fn update_from_slider_only_changes_brightness() {
    let mut state = ColorPickerState::new();
    state.hsb = Hsb::new(240.0, 1.0, 1.0);
    state.selected_color = state.hsb.to_color();

    let _ = reduce(
        &mut state,
        ColorPickerAction::UpdateFromSlider { brightness: 0.5 },
    );

    assert_eq!(state.hsb, Hsb::new(240.0, 1.0, 0.5));
}

#[test]
fn initialize_and_slider_position_actions_are_supported() {
    let mut state = ColorPickerState::new();

    let initialize = reduce(&mut state, ColorPickerAction::Initialize);
    assert!(initialize.is_none());

    let _ = reduce(
        &mut state,
        ColorPickerAction::SetValueSliderPosition {
            position: ColorPickerDock::Left,
        },
    );
    assert_eq!(state.value_slider_position, ColorPickerDock::Left);

    let _ = reduce(
        &mut state,
        ColorPickerAction::SetValueSliderPosition {
            position: ColorPickerDock::Top,
        },
    );
    assert_eq!(state.value_slider_position, ColorPickerDock::Top);

    let _ = reduce(
        &mut state,
        ColorPickerAction::SetValueSliderPosition {
            position: ColorPickerDock::Right,
        },
    );
    assert_eq!(state.value_slider_position, ColorPickerDock::Right);
}
