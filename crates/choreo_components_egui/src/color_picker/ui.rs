use egui::Ui;

use super::actions::ColorPickerAction;
use super::state::ColorPickerDock;
use super::state::ColorPickerState;

pub fn draw(ui: &mut Ui, state: &ColorPickerState) -> Vec<ColorPickerAction> {
    let mut actions: Vec<ColorPickerAction> = Vec::new();

    ui.heading("Color Picker");

    let mut selected_color = state.selected_color;
    if ui.color_edit_button_srgba(&mut selected_color).changed() {
        actions.push(ColorPickerAction::SetColor {
            color: selected_color,
        });
    }

    let mut hue = state.hsb.hue;
    let mut saturation = state.hsb.saturation;
    let mut brightness = state.hsb.brightness;

    if ui
        .add(egui::Slider::new(&mut hue, 0.0..=360.0).text("Hue"))
        .changed()
        || ui
            .add(egui::Slider::new(&mut saturation, 0.0..=1.0).text("Saturation"))
            .changed()
    {
        actions.push(ColorPickerAction::UpdateFromWheel { hue, saturation });
    }

    let slider_response = if matches!(
        state.value_slider_position,
        ColorPickerDock::Left | ColorPickerDock::Right
    ) {
        ui.add(egui::Slider::new(&mut brightness, 0.0..=1.0).text("Brightness").vertical())
    } else {
        ui.add(egui::Slider::new(&mut brightness, 0.0..=1.0).text("Brightness"))
    };

    if slider_response.changed() {
        actions.push(ColorPickerAction::UpdateFromSlider { brightness });
    }

    ui.horizontal(|ui| {
        ui.label("Value slider:");

        if ui
            .selectable_label(
                matches!(state.value_slider_position, ColorPickerDock::Left),
                "Left",
            )
            .clicked()
        {
            actions.push(ColorPickerAction::SetValueSliderPosition {
                position: ColorPickerDock::Left,
            });
        }

        if ui
            .selectable_label(
                matches!(state.value_slider_position, ColorPickerDock::Top),
                "Top",
            )
            .clicked()
        {
            actions.push(ColorPickerAction::SetValueSliderPosition {
                position: ColorPickerDock::Top,
            });
        }

        if ui
            .selectable_label(
                matches!(state.value_slider_position, ColorPickerDock::Right),
                "Right",
            )
            .clicked()
        {
            actions.push(ColorPickerAction::SetValueSliderPosition {
                position: ColorPickerDock::Right,
            });
        }

        if ui
            .selectable_label(
                matches!(state.value_slider_position, ColorPickerDock::Bottom),
                "Bottom",
            )
            .clicked()
        {
            actions.push(ColorPickerAction::SetValueSliderPosition {
                position: ColorPickerDock::Bottom,
            });
        }
    });

    if ui.button("Initialize").clicked() {
        actions.push(ColorPickerAction::Initialize);
    }

    actions
}
