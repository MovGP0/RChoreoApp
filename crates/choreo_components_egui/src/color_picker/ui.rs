use egui::Color32;
use egui::Rect;
use egui::Sense;
use egui::Shape;
use egui::Stroke;
use egui::Ui;
use egui::ecolor::Hsva;
use egui::epaint::Mesh;
use egui::epaint::Vertex;
use egui::pos2;
use egui::vec2;
use egui_material3::MaterialButton;
use egui_material3::MaterialSlider;

use super::actions::ColorPickerAction;
use super::state::ColorPickerDock;
use super::state::ColorPickerState;

const SLIDER_EXTENT_PX: f32 = 64.0;
const SLIDER_SPACING_PX: f32 = 8.0;
const SLIDER_LEFT_INSET_PX: f32 = 6.0;
const SLIDER_RIGHT_INSET_PX: f32 = 14.0;

pub fn draw(ui: &mut Ui, state: &ColorPickerState) -> Vec<ColorPickerAction> {
    let mut actions: Vec<ColorPickerAction> = Vec::new();

    let mut brightness = state.hsb.brightness;
    draw_dock_layout(ui, state, &mut brightness, &mut actions);

    ui.horizontal(|ui| {
        ui.label("Value slider:");

        let left_label = if matches!(state.value_slider_position, ColorPickerDock::Left) {
            "Left*"
        } else {
            "Left"
        };
        if ui.add(MaterialButton::new(left_label)).clicked() {
            actions.push(ColorPickerAction::SetValueSliderPosition {
                position: ColorPickerDock::Left,
            });
        }

        let top_label = if matches!(state.value_slider_position, ColorPickerDock::Top) {
            "Top*"
        } else {
            "Top"
        };
        if ui.add(MaterialButton::new(top_label)).clicked() {
            actions.push(ColorPickerAction::SetValueSliderPosition {
                position: ColorPickerDock::Top,
            });
        }

        let right_label = if matches!(state.value_slider_position, ColorPickerDock::Right) {
            "Right*"
        } else {
            "Right"
        };
        if ui.add(MaterialButton::new(right_label)).clicked() {
            actions.push(ColorPickerAction::SetValueSliderPosition {
                position: ColorPickerDock::Right,
            });
        }

        let bottom_label = if matches!(state.value_slider_position, ColorPickerDock::Bottom) {
            "Bottom*"
        } else {
            "Bottom"
        };
        if ui.add(MaterialButton::new(bottom_label)).clicked() {
            actions.push(ColorPickerAction::SetValueSliderPosition {
                position: ColorPickerDock::Bottom,
            });
        }
    });

    actions
}

fn draw_dock_layout(
    ui: &mut Ui,
    state: &ColorPickerState,
    brightness: &mut f64,
    actions: &mut Vec<ColorPickerAction>,
) {
    match state.value_slider_position {
        ColorPickerDock::Top => {
            draw_horizontal_brightness_slider(ui, brightness, actions);
            ui.add_space(SLIDER_SPACING_PX);
            draw_wheel(ui, state, actions);
        }
        ColorPickerDock::Bottom => {
            draw_wheel(ui, state, actions);
            ui.add_space(SLIDER_SPACING_PX);
            draw_horizontal_brightness_slider(ui, brightness, actions);
        }
        ColorPickerDock::Left => {
            ui.horizontal(|ui| {
                draw_vertical_brightness_slider(ui, state, brightness, false, actions);
                ui.add_space(SLIDER_SPACING_PX);
                draw_wheel(ui, state, actions);
            });
        }
        ColorPickerDock::Right => {
            ui.horizontal(|ui| {
                draw_wheel(ui, state, actions);
                ui.add_space(SLIDER_SPACING_PX);
                draw_vertical_brightness_slider(ui, state, brightness, true, actions);
            });
        }
    }
}

fn draw_horizontal_brightness_slider(
    ui: &mut Ui,
    brightness: &mut f64,
    actions: &mut Vec<ColorPickerAction>,
) {
    let slider_width =
        (ui.available_width() - SLIDER_LEFT_INSET_PX - SLIDER_RIGHT_INSET_PX).max(0.0);
    ui.allocate_ui(vec2(ui.available_width(), SLIDER_EXTENT_PX), |ui| {
        ui.horizontal(|ui| {
            ui.add_space(SLIDER_LEFT_INSET_PX);
            let mut slider_value = *brightness as f32;
            let response = ui.add(
                MaterialSlider::new(&mut slider_value, 0.0..=1.0)
                    .show_value(false)
                    .width(slider_width),
            );
            ui.add_space(SLIDER_RIGHT_INSET_PX);
            if response.changed() {
                *brightness = f64::from(slider_value);
                actions.push(ColorPickerAction::UpdateFromSlider {
                    brightness: *brightness,
                });
            }
        });
    });
}

fn draw_vertical_brightness_slider(
    ui: &mut Ui,
    state: &ColorPickerState,
    brightness: &mut f64,
    invert_direction: bool,
    actions: &mut Vec<ColorPickerAction>,
) {
    // MaterialSlider in egui-material3 is horizontal-only in the current crate version.
    // Keep egui's vertical slider to preserve left/right dock parity with the source component.
    let mut slider_value = slider_value_from_brightness(*brightness, invert_direction);
    let response = ui.add_sized(
        vec2(SLIDER_EXTENT_PX, state.wheel_minimum_height),
        egui::Slider::new(&mut slider_value, 0.0..=1.0)
            .text("Brightness")
            .vertical(),
    );
    if response.changed() {
        *brightness = brightness_from_slider_value(slider_value, invert_direction);
        actions.push(ColorPickerAction::UpdateFromSlider {
            brightness: *brightness,
        });
    }
}

#[must_use]
pub(crate) fn slider_value_from_brightness(brightness: f64, invert_direction: bool) -> f64 {
    if invert_direction {
        1.0 - brightness
    } else {
        brightness
    }
}

#[must_use]
pub(crate) fn brightness_from_slider_value(slider_value: f64, invert_direction: bool) -> f64 {
    if invert_direction {
        1.0 - slider_value
    } else {
        slider_value
    }
}

fn draw_wheel(ui: &mut Ui, state: &ColorPickerState, actions: &mut Vec<ColorPickerAction>) {
    let minimum_size = vec2(state.wheel_minimum_width, state.wheel_minimum_height);
    let (alloc_rect, response) = ui.allocate_at_least(minimum_size, Sense::click_and_drag());
    let wheel_size = alloc_rect.width().min(alloc_rect.height());
    let wheel_rect = Rect::from_center_size(alloc_rect.center(), vec2(wheel_size, wheel_size));
    let wheel_center = wheel_rect.center();
    let wheel_radius = wheel_rect.width() * 0.5;

    let mut mesh = Mesh::default();
    let center_vertex_index = mesh.vertices.len() as u32;
    mesh.vertices.push(Vertex {
        pos: wheel_center,
        uv: pos2(0.0, 0.0),
        color: Color32::WHITE,
    });

    let segment_count: usize = 96;
    for segment in 0..=segment_count {
        let angle = std::f32::consts::TAU * segment as f32 / segment_count as f32;
        let hue = f32::to_degrees(angle).rem_euclid(360.0) / 360.0;
        let direction = vec2(angle.cos(), angle.sin());
        let outer = wheel_center + direction * wheel_radius;
        let color = Color32::from(Hsva::new(hue, 1.0, 1.0, 1.0));
        mesh.vertices.push(Vertex {
            pos: outer,
            uv: pos2(0.0, 0.0),
            color,
        });
    }

    for segment in 0..segment_count {
        let first_outer_index = center_vertex_index + 1 + segment as u32;
        let second_outer_index = first_outer_index + 1;
        mesh.indices.push(center_vertex_index);
        mesh.indices.push(first_outer_index);
        mesh.indices.push(second_outer_index);
    }

    ui.painter().add(Shape::mesh(mesh));
    let thumb_angle = state.hsb.hue.to_radians() as f32;
    let thumb_distance = (state.hsb.saturation as f32).clamp(0.0, 1.0) * wheel_radius;
    let thumb_center = wheel_center + vec2(thumb_angle.cos(), thumb_angle.sin()) * thumb_distance;
    let thumb_radius = (state.selection_thumb_size * 0.5).max(1.0);

    ui.painter()
        .circle_filled(thumb_center, thumb_radius, state.selected_color);
    ui.painter().circle_stroke(
        thumb_center,
        thumb_radius,
        Stroke::new(
            state.selection_stroke_thickness,
            state.selection_stroke_color,
        ),
    );

    if (response.clicked() || response.dragged()) && response.interact_pointer_pos().is_some() {
        let pointer = response.interact_pointer_pos().unwrap_or(wheel_center);
        actions.push(ColorPickerAction::UpdateFromWheelPoint {
            x: pointer.x,
            y: pointer.y,
            center_x: wheel_center.x,
            center_y: wheel_center.y,
            radius_px: wheel_radius,
        });
    }
}
