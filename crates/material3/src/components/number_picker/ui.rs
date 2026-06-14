use egui::Align2;
use egui::Color32;
use egui::CornerRadius;
use egui::Image;
use egui::Label;
use egui::Rect;
use egui::Response;
use egui::Sense;
use egui::Stroke;
use egui::StrokeKind;
use egui::Ui;
use egui::vec2;

use crate::components::icon::centered_icon_rect;
use crate::components::icon::paint_icon;
use crate::styling::material_palette::MaterialPalette;
use crate::styling::material_palette::material_palette_for_visuals;
use crate::styling::material_style_metrics::material_style_metrics;
use crate::styling::material_typography::TypographyRole;
use crate::styling::material_typography::font_id_for_role;
use crate::styling::material_typography::rich_text_for_role;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NumberPickerLayoutMetrics {
    pub label_width: f32,
    pub controls_width: f32,
    pub control_height: f32,
    pub value_width: f32,
    pub value_content_height: f32,
    pub button_size: f32,
    pub spacing: f32,
}

#[cfg(test)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NumberPickerControlRole {
    Value,
    IncrementButton,
    DecrementButton,
}

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

#[must_use]
pub fn number_picker_layout_metrics(available_width: f32) -> NumberPickerLayoutMetrics {
    let metrics = material_style_metrics();
    let spacing = metrics.spacings.spacing_8;
    let button_size = metrics.sizes.size_40;
    let value_width = metrics.sizes.size_72;
    let control_height = metrics.sizes.size_40;
    let vertical_value_padding = metrics.paddings.padding_8 * 2.0;
    let controls_width = value_width + button_size * 2.0 + spacing * 2.0;
    let label_width = (available_width - controls_width - spacing).max(0.0);

    NumberPickerLayoutMetrics {
        label_width,
        controls_width,
        control_height,
        value_width,
        value_content_height: (control_height - vertical_value_padding).max(0.0),
        button_size,
        spacing,
    }
}

#[cfg(test)]
const fn number_picker_control_order() -> [NumberPickerControlRole; 3] {
    [
        NumberPickerControlRole::Value,
        NumberPickerControlRole::IncrementButton,
        NumberPickerControlRole::DecrementButton,
    ]
}

pub fn draw(ui: &mut Ui, state: NumberPickerUiState<'_>) -> Option<i32> {
    let style_metrics = material_style_metrics();
    let layout_metrics = number_picker_layout_metrics(ui.available_width());
    let current_value = clamped_value(state.value, state.minimum, state.maximum);
    let can_decrease = state.enabled
        && decrease_value(current_value, state.minimum, state.maximum, state.step).is_some();
    let can_increase = state.enabled
        && increase_value(current_value, state.minimum, state.maximum, state.step).is_some();
    let mut changed_value = None;

    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = layout_metrics.spacing;
        let label = Label::new(
            rich_text_for_role(state.label, TypographyRole::BodyMedium)
                .color(ui.visuals().weak_text_color()),
        )
        .truncate();
        let _ = ui.add_sized(
            vec2(layout_metrics.label_width, layout_metrics.control_height),
            label,
        );

        let (controls_rect, _) = ui.allocate_exact_size(
            vec2(layout_metrics.controls_width, layout_metrics.control_height),
            Sense::hover(),
        );

        let value_rect = Rect::from_min_size(
            controls_rect.min,
            vec2(layout_metrics.value_width, layout_metrics.control_height),
        );
        let increment_rect = Rect::from_min_size(
            value_rect.right_top() + vec2(layout_metrics.spacing, 0.0),
            vec2(layout_metrics.button_size, layout_metrics.button_size),
        );
        let decrement_rect = Rect::from_min_size(
            increment_rect.right_top() + vec2(layout_metrics.spacing, 0.0),
            vec2(layout_metrics.button_size, layout_metrics.button_size),
        );

        paint_number_picker_value(
            ui,
            value_rect,
            current_value,
            state.enabled,
            style_metrics.strokes.outline,
            style_metrics.corner_radii.border_radius_8 as u8,
        );

        if number_picker_icon_button(
            ui,
            increment_rect,
            number_picker_increment_icon(),
            can_increase,
        )
        .clicked()
            && can_increase
        {
            changed_value = increase_value(current_value, state.minimum, state.maximum, state.step);
        }

        if number_picker_icon_button(
            ui,
            decrement_rect,
            number_picker_decrement_icon(),
            can_decrease,
        )
        .clicked()
            && can_decrease
        {
            changed_value = decrease_value(current_value, state.minimum, state.maximum, state.step);
        }
    });

    changed_value
}

fn paint_number_picker_value(
    ui: &Ui,
    rect: Rect,
    value: i32,
    enabled: bool,
    outline_width: f32,
    corner_radius: u8,
) {
    ui.painter().rect_filled(
        rect,
        CornerRadius::same(corner_radius),
        value_fill(ui, enabled),
    );
    ui.painter().rect_stroke(
        rect,
        CornerRadius::same(corner_radius),
        Stroke::new(
            outline_width,
            ui.visuals().widgets.noninteractive.bg_stroke.color,
        ),
        StrokeKind::Inside,
    );
    ui.painter().text(
        rect.center(),
        Align2::CENTER_CENTER,
        value.to_string(),
        font_id_for_role(TypographyRole::TitleMedium),
        value_text_color(ui, enabled),
    );
}

fn number_picker_icon_button(
    ui: &mut Ui,
    rect: Rect,
    icon: Image<'static>,
    enabled: bool,
) -> Response {
    let response = ui.allocate_rect(rect, Sense::click());
    let palette = material_palette_for_visuals(ui.visuals());
    let state_color = number_picker_icon_button_state_color(palette);
    let pressed = enabled && response.is_pointer_button_down_on();
    let press_progress =
        ui.ctx()
            .animate_bool_with_time(response.id.with("press_ripple"), pressed, 0.16);
    if press_progress > 0.0 {
        ui.painter().circle_filled(
            rect.center(),
            number_picker_icon_button_ripple_radius(rect, press_progress),
            state_color.gamma_multiply(palette.state_layer_opacities.press),
        );
    } else if enabled && response.hovered() {
        ui.painter().circle_filled(
            rect.center(),
            number_picker_icon_button_max_state_radius(rect),
            state_color.gamma_multiply(palette.state_layer_opacities.hover),
        );
    }

    let icon_tint = if enabled {
        state_color
    } else {
        state_color.gamma_multiply(0.38)
    };
    paint_icon(
        ui,
        &icon,
        centered_icon_rect(rect, vec2(24.0, 24.0)),
        icon_tint,
    );

    response
}

fn number_picker_icon_button_state_color(palette: MaterialPalette) -> Color32 {
    palette.on_surface_variant
}

#[cfg(test)]
fn number_picker_icon_button_state_opacity(
    palette: MaterialPalette,
    enabled: bool,
    hovered: bool,
    pressed: bool,
) -> f32 {
    if !enabled {
        return 0.0;
    }

    if pressed {
        return palette.state_layer_opacities.press;
    }

    if hovered {
        return palette.state_layer_opacities.hover;
    }

    0.0
}

fn number_picker_icon_button_max_state_radius(rect: Rect) -> f32 {
    rect.width().min(rect.height()) * 0.5
}

fn number_picker_icon_button_ripple_radius(rect: Rect, progress: f32) -> f32 {
    number_picker_icon_button_max_state_radius(rect) * progress.clamp(0.0, 1.0)
}

fn number_picker_decrement_icon() -> Image<'static> {
    Image::new(egui::include_image!("../../../assets/icons/Minus.svg"))
}

fn number_picker_increment_icon() -> Image<'static> {
    Image::new(egui::include_image!("../../../assets/icons/Plus.svg"))
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

#[cfg(test)]
mod tests {
    use super::NumberPickerControlRole;
    use super::number_picker_control_order;
    use super::number_picker_icon_button_max_state_radius;
    use super::number_picker_icon_button_ripple_radius;
    use super::number_picker_icon_button_state_color;
    use super::number_picker_icon_button_state_opacity;
    use super::number_picker_layout_metrics;
    use crate::styling::material_palette::MaterialPalette;
    use crate::styling::material_style_metrics::material_style_metrics;
    use egui::pos2;
    use egui::vec2;

    #[test]
    fn number_picker_controls_keep_stepper_buttons_to_the_right_of_value() {
        assert_eq!(
            number_picker_control_order(),
            [
                NumberPickerControlRole::Value,
                NumberPickerControlRole::IncrementButton,
                NumberPickerControlRole::DecrementButton,
            ]
        );
    }

    #[test]
    fn number_picker_value_content_height_accounts_for_frame_padding() {
        let metrics = material_style_metrics();
        let layout = number_picker_layout_metrics(326.0);
        let vertical_padding = metrics.paddings.padding_8 * 2.0;

        assert_eq!(layout.control_height, metrics.sizes.size_40);
        assert_eq!(
            layout.value_content_height + vertical_padding,
            layout.control_height
        );
    }

    #[test]
    fn number_picker_button_pressed_state_uses_distinct_opacity_from_hover() {
        let palette = MaterialPalette::light();
        let hover_opacity = number_picker_icon_button_state_opacity(palette, true, true, false);
        let pressed_opacity = number_picker_icon_button_state_opacity(palette, true, true, true);

        assert_eq!(hover_opacity, palette.state_layer_opacities.hover);
        assert_eq!(pressed_opacity, palette.state_layer_opacities.press);
        assert_ne!(hover_opacity, pressed_opacity);
    }

    #[test]
    fn number_picker_button_ripple_radius_expands_with_progress() {
        let rect = egui::Rect::from_min_size(pos2(0.0, 0.0), vec2(40.0, 40.0));

        assert_eq!(number_picker_icon_button_ripple_radius(rect, 0.0), 0.0);
        assert_eq!(
            number_picker_icon_button_ripple_radius(rect, 1.0),
            number_picker_icon_button_max_state_radius(rect)
        );
        assert!(
            number_picker_icon_button_ripple_radius(rect, 0.5)
                < number_picker_icon_button_max_state_radius(rect)
        );
    }

    #[test]
    fn number_picker_button_state_color_follows_light_and_dark_palettes() {
        let light_palette = MaterialPalette::light();
        let dark_palette = MaterialPalette::dark();

        assert!(
            color_luminance(number_picker_icon_button_state_color(light_palette))
                < color_luminance(light_palette.surface)
        );
        assert!(
            color_luminance(number_picker_icon_button_state_color(dark_palette))
                > color_luminance(dark_palette.surface)
        );
        assert_ne!(
            number_picker_icon_button_state_color(light_palette),
            number_picker_icon_button_state_color(dark_palette)
        );
    }

    fn color_luminance(color: egui::Color32) -> f32 {
        let red = f32::from(color.r()) / 255.0;
        let green = f32::from(color.g()) / 255.0;
        let blue = f32::from(color.b()) / 255.0;

        (0.2126 * red) + (0.7152 * green) + (0.0722 * blue)
    }
}
