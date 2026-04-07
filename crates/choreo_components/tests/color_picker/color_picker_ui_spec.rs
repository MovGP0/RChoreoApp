use egui::Color32;

use super::state;
use super::ui;

macro_rules! check_eq {
    ($errors:expr, $left:expr, $right:expr) => {
        if $left != $right {
            $errors.push(format!(
                "{} != {} (left = {:?}, right = {:?})",
                stringify!($left),
                stringify!($right),
                $left,
                $right
            ));
        }
    };
}

macro_rules! check {
    ($errors:expr, $condition:expr) => {
        if !$condition {
            $errors.push(format!("condition failed: {}", stringify!($condition)));
        }
    };
}

fn assert_no_errors(errors: Vec<String>) {
    assert!(
        errors.is_empty(),
        "Assertion failures:\n{}",
        errors.join("\n")
    );
}

#[test]
fn state_for_color_keeps_selected_color_and_syncs_hsb() {
    let selected_color = Color32::from_rgb(12, 128, 200);

    let state = ui::state_for_color(selected_color);

    let mut errors = Vec::new();

    check_eq!(errors, state.selected_color, selected_color);
    check_eq!(errors, state.hsb, state::Hsb::from_color(selected_color));

    assert_no_errors(errors);
}

#[test]
fn min_size_for_bottom_dock_matches_shared_component_contract() {
    let state = state::ColorPickerState::new();

    let min_size = ui::min_size_for_state(&state);

    assert_eq!(
        min_size,
        egui::vec2(
            state.wheel_minimum_width,
            state.wheel_minimum_height + 64.0 + 8.0,
        )
    );
}

#[test]
fn min_size_for_side_dock_matches_shared_component_contract() {
    let mut state = state::ColorPickerState::new();
    state.value_slider_position = state::ColorPickerDock::Left;

    let min_size = ui::min_size_for_state(&state);

    assert_eq!(
        min_size,
        egui::vec2(
            state.wheel_minimum_width + 64.0 + 8.0,
            state.wheel_minimum_height,
        )
    );
}

#[test]
fn draw_uses_selection_thumb_size_and_stroke_settings() {
    let context = egui::Context::default();
    let mut state = state::ColorPickerState::new();
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

    let mut errors = Vec::new();

    check!(errors, saw_thumb_fill);
    check!(errors, saw_thumb_stroke);

    assert_no_errors(errors);
}

#[test]
fn vertical_slider_mapping_is_identity_for_left_dock() {
    let brightness = 0.25;
    let slider_value = ui::slider_value_from_brightness(brightness, false);
    let mapped_back = ui::brightness_from_slider_value(slider_value, false);

    let mut errors = Vec::new();

    check!(
        errors,
        (slider_value - 0.25).abs() < f64::EPSILON
    );
    check!(
        errors,
        (mapped_back - brightness).abs() < f64::EPSILON
    );

    assert_no_errors(errors);
}

#[test]
fn vertical_slider_mapping_is_inverted_for_right_dock() {
    let brightness = 0.25;
    let slider_value = ui::slider_value_from_brightness(brightness, true);
    let mapped_back = ui::brightness_from_slider_value(slider_value, true);

    let mut errors = Vec::new();

    check!(
        errors,
        (slider_value - 0.75).abs() < f64::EPSILON
    );
    check!(
        errors,
        (mapped_back - brightness).abs() < f64::EPSILON
    );

    assert_no_errors(errors);
}
