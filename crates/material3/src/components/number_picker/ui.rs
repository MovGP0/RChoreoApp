use egui::Align;
use egui::Color32;
use egui::CornerRadius;
use egui::Frame;
use egui::Layout;
use egui::Margin;
use egui::Stroke;
use egui::Ui;
use egui::vec2;
use egui_material3::MaterialIconButton;

use crate::icons::UiIconKey;
use crate::icons::icon as ui_icon;
use crate::styling::material_style_metrics::material_style_metrics;
use crate::styling::material_typography::TypographyRole;
use crate::styling::material_typography::rich_text_for_role;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NumberPickerUiState<'a> {
    pub label: &'a str,
    pub value: i32,
    pub minimum: i32,
    pub maximum: i32,
    pub step: i32,
    pub enabled: bool,
}

#[must_use]
pub const fn normalized_step(step: i32) -> i32 {
    if step < 1 { 1 } else { step }
}

#[must_use]
pub fn clamped_value(value: i32, minimum: i32, maximum: i32) -> i32 {
    let (minimum, maximum) = normalized_bounds(minimum, maximum);
    value.clamp(minimum, maximum)
}

#[must_use]
pub fn decrease_value(value: i32, minimum: i32, maximum: i32, step: i32) -> Option<i32> {
    let clamped = clamped_value(value, minimum, maximum);
    let next = clamped_value(clamped - normalized_step(step), minimum, maximum);
    if next != clamped { Some(next) } else { None }
}

#[must_use]
pub fn increase_value(value: i32, minimum: i32, maximum: i32, step: i32) -> Option<i32> {
    let clamped = clamped_value(value, minimum, maximum);
    let next = clamped_value(clamped + normalized_step(step), minimum, maximum);
    if next != clamped { Some(next) } else { None }
}

pub fn draw(ui: &mut Ui, state: NumberPickerUiState<'_>) -> Option<i32> {
    let metrics = material_style_metrics();
    let current_value = clamped_value(state.value, state.minimum, state.maximum);
    let can_decrease = state.enabled
        && decrease_value(current_value, state.minimum, state.maximum, state.step).is_some();
    let can_increase = state.enabled
        && increase_value(current_value, state.minimum, state.maximum, state.step).is_some();
    let decrement_icon = ui_icon(UiIconKey::NumberPickerDecrement);
    let increment_icon = ui_icon(UiIconKey::NumberPickerIncrement);
    let mut changed_value = None;

    ui.horizontal(|ui| {
        ui.label(
            rich_text_for_role(state.label, TypographyRole::BodyMedium)
                .color(ui.visuals().weak_text_color()),
        );
        ui.add_space(ui.available_width().max(0.0));

        if ui
            .add_enabled(
                can_decrease,
                MaterialIconButton::standard(decrement_icon.token).svg_data(decrement_icon.svg),
            )
            .clicked()
        {
            changed_value = decrease_value(current_value, state.minimum, state.maximum, state.step);
        }

        Frame::new()
            .fill(value_fill(ui, state.enabled))
            .stroke(Stroke::new(
                metrics.strokes.outline,
                ui.visuals().widgets.noninteractive.bg_stroke.color,
            ))
            .corner_radius(CornerRadius::same(
                metrics.corner_radii.border_radius_8 as u8,
            ))
            .inner_margin(Margin::symmetric(0, metrics.paddings.padding_8 as i8))
            .show(ui, |ui| {
                ui.set_min_size(vec2(metrics.sizes.size_72, metrics.sizes.size_40));
                ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                    ui.add_space(ui.available_width().max(0.0) / 2.0);
                    ui.label(
                        rich_text_for_role(current_value.to_string(), TypographyRole::TitleMedium)
                            .color(value_text_color(ui, state.enabled)),
                    );
                    ui.add_space(ui.available_width().max(0.0) / 2.0);
                });
            });

        if ui
            .add_enabled(
                can_increase,
                MaterialIconButton::standard(increment_icon.token).svg_data(increment_icon.svg),
            )
            .clicked()
        {
            changed_value = increase_value(current_value, state.minimum, state.maximum, state.step);
        }
    });

    changed_value
}

fn normalized_bounds(minimum: i32, maximum: i32) -> (i32, i32) {
    if minimum <= maximum {
        (minimum, maximum)
    } else {
        (maximum, minimum)
    }
}

fn value_fill(ui: &Ui, enabled: bool) -> Color32 {
    if enabled {
        ui.visuals().widgets.noninteractive.bg_fill
    } else {
        ui.visuals().faint_bg_color
    }
}

fn value_text_color(ui: &Ui, enabled: bool) -> Color32 {
    if enabled {
        ui.visuals().strong_text_color()
    } else {
        ui.visuals().weak_text_color()
    }
}
