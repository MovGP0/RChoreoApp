use egui::Color32;

use std::f64::consts::FRAC_1_SQRT_2;

use super::actions::ColorPickerAction;
use super::reducer::reduce;
use super::state::ColorPickerDock;
use super::state::ColorPickerState;
use super::state::Hsb;
use super::ui;

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
fn update_from_wheel_point_maps_angle_and_clamps_saturation() {
    let mut state = ColorPickerState::new();
    state.hsb = Hsb::new(0.0, 0.0, 0.5);
    state.selected_color = state.hsb.to_color();

    let _ = reduce(
        &mut state,
        ColorPickerAction::UpdateFromWheelPoint {
            x: 220.0,
            y: 100.0,
            center_x: 100.0,
            center_y: 100.0,
            radius_px: 100.0,
        },
    );

    assert!((state.hsb.hue - 0.0).abs() < f64::EPSILON);
    assert!((state.hsb.saturation - 1.0).abs() < f64::EPSILON);
    assert!((state.hsb.brightness - 0.5).abs() < f64::EPSILON);
}

#[test]
fn update_from_wheel_point_wraps_negative_angles_to_hue_range() {
    let mut state = ColorPickerState::new();
    state.hsb = Hsb::new(0.0, 0.0, 1.0);
    state.selected_color = state.hsb.to_color();

    let _ = reduce(
        &mut state,
        ColorPickerAction::UpdateFromWheelPoint {
            x: 200.0,
            y: 0.0,
            center_x: 100.0,
            center_y: 100.0,
            radius_px: 200.0,
        },
    );

    assert!((state.hsb.hue - 315.0).abs() < 0.001);
    assert!((state.hsb.saturation - FRAC_1_SQRT_2).abs() < 0.001);
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
fn slider_position_actions_are_supported() {
    let mut state = ColorPickerState::new();

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

#[test]
fn draw_uses_selection_thumb_size_and_stroke_settings() {
    let context = egui::Context::default();
    let mut state = ColorPickerState::new();
    state.selection_thumb_size = 24.0;
    state.selection_stroke_thickness = 3.0;
    state.selection_stroke_color = Color32::from_rgb(4, 5, 6);

    let output = context.run(egui::RawInput::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let _ = ui::draw(ui, &state);
        });
    });

    let mut saw_thumb_fill = false;
    let mut saw_thumb_stroke = false;
    for clipped in output.shapes {
        if let egui::Shape::Circle(circle) = clipped.shape {
            if (circle.radius - (state.selection_thumb_size * 0.5)).abs() < 0.01
                && circle.fill == state.selected_color
            {
                saw_thumb_fill = true;
            }
            if (circle.radius - (state.selection_thumb_size * 0.5)).abs() < 0.01
                && (circle.stroke.width - state.selection_stroke_thickness).abs() < 0.01
                && circle.stroke.color == state.selection_stroke_color
            {
                saw_thumb_stroke = true;
            }
        }
    }

    assert!(saw_thumb_fill);
    assert!(saw_thumb_stroke);
}
