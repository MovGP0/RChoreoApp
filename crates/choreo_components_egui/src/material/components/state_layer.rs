use egui::Color32;
use egui::CornerRadius;
use egui::Painter;
use egui::Pos2;
use egui::Rect;
use egui::Response;
use egui::vec2;
use std::f32::consts::SQRT_2;

use crate::material::components::tooltip::ToolTip;
use crate::material::styling::material_palette::MaterialPalette;
use crate::material::styling::material_palette::material_palette_for_visuals;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StateLayerStyle<'a> {
    pub color: Color32,
    pub border_radius: f32,
    pub transparent_background: bool,
    pub display_background: bool,
    pub enabled: bool,
    pub has_focus: bool,
    pub has_hover: bool,
    pub pressed: bool,
    pub enter_pressed: bool,
    pub pressed_position: Pos2,
    pub clip_ripple: bool,
    pub disable_hover: bool,
    pub tooltip: &'a str,
    pub tooltip_offset: f32,
}

impl<'a> StateLayerStyle<'a> {
    #[must_use]
    pub fn for_ui(ui: &egui::Ui) -> Self {
        let palette = material_palette_for_visuals(ui.visuals());
        Self {
            color: palette.on_surface,
            border_radius: 0.0,
            transparent_background: false,
            display_background: true,
            enabled: true,
            has_focus: false,
            has_hover: false,
            pressed: false,
            enter_pressed: false,
            pressed_position: Pos2::ZERO,
            clip_ripple: true,
            disable_hover: false,
            tooltip: "",
            tooltip_offset: 0.0,
        }
    }
}

#[must_use]
pub fn state_layer_opacity(style: StateLayerStyle<'_>, palette: MaterialPalette) -> f32 {
    if !style.enabled && style.display_background {
        return palette.state_layer_opacity_focus;
    }
    if style.enabled && style.pressed_or_enter() {
        return palette.state_layer_opacity_press;
    }
    if style.enabled && style.has_focus {
        return palette.state_layer_opacity_focus;
    }
    if style.enabled && style.has_hover && !style.disable_hover {
        return palette.state_layer_opacity_hover;
    }
    0.0
}

pub fn paint_state_layer(painter: &Painter, rect: Rect, style: StateLayerStyle<'_>, palette: MaterialPalette) {
    let opacity = state_layer_opacity(style, palette);
    if opacity <= 0.0 {
        return;
    }

    let rounding = CornerRadius::same(style.border_radius.round() as u8);
    let base_color = if style.transparent_background {
        Color32::TRANSPARENT
    } else {
        style.color
    };
    painter.rect_filled(rect, rounding, base_color.gamma_multiply(opacity));

    if style.enabled && style.pressed_or_enter() {
        let ripple_radius = rect.width().max(rect.height()) * SQRT_2;
        if style.clip_ripple {
            painter.with_clip_rect(rect).circle_filled(
                style.pressed_position,
                ripple_radius,
                style.color.gamma_multiply(palette.state_layer_opacity_press),
            );
        } else {
            painter.circle_filled(
                style.pressed_position,
                ripple_radius,
                style.color.gamma_multiply(palette.state_layer_opacity_press),
            );
        }
    }
}

#[must_use]
pub fn tooltip_anchor_for_response(response: &Response, style: StateLayerStyle<'_>) -> Pos2 {
    response.rect.left_bottom() + vec2(0.0, style.tooltip_offset)
}

pub fn apply_tooltip(ui: &mut egui::Ui, response: Response, style: StateLayerStyle<'_>) -> Response {
    if style.tooltip.is_empty() || style.disable_hover || style.pressed_or_enter() || !response.hovered() {
        return response;
    }
    let anchor = tooltip_anchor_for_response(&response, style);
    let _ = ToolTip::new(style.tooltip).show(ui, anchor);
    response
}

pub fn paint_state_layer_for_response(
    ui: &egui::Ui,
    response: &Response,
    style: StateLayerStyle<'_>,
) {
    let mut updated = style;
    updated.has_focus = response.has_focus();
    updated.has_hover = response.hovered();
    updated.pressed = response.is_pointer_button_down_on();
    let pointer = ui
        .input(|input| input.pointer.interact_pos())
        .unwrap_or(response.rect.center());
    updated.pressed_position = pointer;
    paint_state_layer(
        ui.painter(),
        response.rect,
        updated,
        material_palette_for_visuals(ui.visuals()),
    );
}

impl StateLayerStyle<'_> {
    #[must_use]
    fn pressed_or_enter(self) -> bool {
        self.pressed || self.enter_pressed
    }
}

#[cfg(test)]
mod tests {
    use egui::Color32;
    use egui::Context;
    use egui::Pos2;

    use super::StateLayerStyle;
    use super::apply_tooltip;
    use super::state_layer_opacity;
    use super::tooltip_anchor_for_response;
    use crate::material::styling::material_palette::MaterialPalette;

    #[test]
    fn pressed_state_uses_press_opacity() {
        let palette = MaterialPalette::light();
        let context = Context::default();
        let mut opacity = 0.0;
        let _ = context.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let mut style = StateLayerStyle::for_ui(ui);
                style.color = Color32::WHITE;
                style.pressed = true;
                opacity = state_layer_opacity(style, palette);
            });
        });
        assert_eq!(opacity, palette.state_layer_opacity_press);
    }

    #[test]
    fn disable_hover_suppresses_hover_opacity_and_tooltip() {
        let palette = MaterialPalette::light();
        let context = Context::default();
        let mut opacity = 0.0;
        let _ = context.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let mut style = StateLayerStyle::for_ui(ui);
                style.disable_hover = true;
                style.has_hover = true;
                style.tooltip = "blocked";
                opacity = state_layer_opacity(style, palette);
                let response = ui.label("hover");
                let _ = apply_tooltip(ui, response, style);
            });
        });
        assert_eq!(opacity, 0.0);
    }

    #[test]
    fn tooltip_anchor_applies_vertical_offset() {
        let context = Context::default();
        let mut anchor = Pos2::ZERO;
        let mut bottom = 0.0;
        let _ = context.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let response = ui.label("hover");
                let mut style = StateLayerStyle::for_ui(ui);
                style.tooltip_offset = 24.0;
                bottom = response.rect.bottom();
                anchor = tooltip_anchor_for_response(&response, style);
            });
        });
        assert_eq!(anchor.y - bottom, 24.0);
    }
}
