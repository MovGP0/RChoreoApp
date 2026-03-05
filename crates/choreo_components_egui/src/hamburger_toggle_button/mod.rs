use egui::Color32;
use egui::Pos2;
use egui::Rect;
use egui::Response;
use egui::Sense;
use egui::Shape;
use egui::Stroke;
use egui::Ui;
use egui::Vec2;
use egui::vec2;

use crate::ui_style::material_animations::MaterialAnimation;
use crate::ui_style::material_animations::MaterialAnimationSpec;
use crate::ui_style::material_animations::MaterialAnimations;
use crate::ui_style::material_style_metrics::material_style_metrics;

const CHECKED_ROTATION_DEGREES: f32 = 35.0;

#[must_use]
pub const fn minimum_button_size_token() -> f32 {
    material_style_metrics().sizes.size_40
}

#[must_use]
pub const fn content_padding_token() -> f32 {
    material_style_metrics().paddings.padding_10
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StateOpacityTokens {
    pub disabled: f32,
    pub hover: f32,
    pub pressed: f32,
}

#[must_use]
pub const fn state_opacity_tokens() -> StateOpacityTokens {
    let metrics = material_style_metrics();
    StateOpacityTokens {
        disabled: metrics.state_opacities.content_disabled,
        hover: metrics.state_opacities.hover,
        pressed: metrics.state_opacities.pressed,
    }
}

#[must_use]
pub const fn checked_animation_spec() -> MaterialAnimationSpec {
    MaterialAnimations::spec(MaterialAnimation::Emphasized)
}

#[must_use]
pub const fn state_layer_animation_spec() -> MaterialAnimationSpec {
    MaterialAnimations::spec(MaterialAnimation::Opacity)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HamburgerToggleButtonGeometry {
    pub top_start: Pos2,
    pub top_end: Pos2,
    pub middle_start: Pos2,
    pub middle_end: Pos2,
    pub bottom_start: Pos2,
    pub bottom_end: Pos2,
    pub thickness: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HamburgerToggleButton<'a> {
    checked: bool,
    enabled: bool,
    toggle_on_click: bool,
    tooltip: &'a str,
    size: Option<Vec2>,
}

#[derive(Debug)]
pub struct HamburgerToggleButtonResult {
    pub response: Response,
    pub checked: bool,
}

impl<'a> HamburgerToggleButton<'a> {
    #[must_use]
    pub const fn new(checked: bool) -> Self {
        Self {
            checked,
            enabled: true,
            toggle_on_click: true,
            tooltip: "",
            size: None,
        }
    }

    #[must_use]
    pub const fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    #[must_use]
    pub const fn toggle_on_click(mut self, toggle_on_click: bool) -> Self {
        self.toggle_on_click = toggle_on_click;
        self
    }

    #[must_use]
    pub const fn tooltip(mut self, tooltip: &'a str) -> Self {
        self.tooltip = tooltip;
        self
    }

    #[must_use]
    pub const fn size(mut self, size: Vec2) -> Self {
        self.size = Some(size);
        self
    }

    #[must_use]
    pub fn show(self, ui: &mut Ui) -> HamburgerToggleButtonResult {
        let response = draw_response(ui, self.checked, self.enabled, self.tooltip, self.size);
        let checked = next_checked_state(
            self.checked,
            self.enabled,
            self.toggle_on_click,
            response.clicked(),
        );

        HamburgerToggleButtonResult { response, checked }
    }
}

#[must_use]
pub fn draw(
    ui: &mut Ui,
    checked: bool,
    enabled: bool,
    tooltip: &str,
    size: Option<Vec2>,
) -> Response {
    draw_response(ui, checked, enabled, tooltip, size)
}

fn draw_response(
    ui: &mut Ui,
    checked: bool,
    enabled: bool,
    tooltip: &str,
    size: Option<Vec2>,
) -> Response {
    let desired_size = desired_size(size);
    let sense = if enabled {
        Sense::click()
    } else {
        Sense::hover()
    };
    let (rect, raw_response) = ui.allocate_exact_size(desired_size, sense);
    let response = if tooltip.is_empty() {
        raw_response
    } else {
        raw_response.on_hover_text(tooltip)
    };

    if !ui.is_rect_visible(rect) {
        return response;
    }

    let checked_progress =
        checked_animation_spec().animate_bool(ui.ctx(), response.id.with("checked"), checked);
    let opacity_tokens = state_opacity_tokens();

    let visuals = ui.style().interact(&response);
    let mut bar_color = lerp_color(
        visuals.fg_stroke.color,
        ui.visuals().selection.stroke.color,
        checked_progress,
    );

    if !enabled {
        bar_color = with_opacity(bar_color, opacity_tokens.disabled);
    }

    let is_hovered_or_pressed = response.hovered() || response.is_pointer_button_down_on();
    let hover_progress = state_layer_animation_spec().animate_bool(
        ui.ctx(),
        response.id.with("hover"),
        is_hovered_or_pressed,
    );

    if hover_progress > 0.0 {
        let overlay_color = lerp_color(
            with_opacity(
                ui.visuals().widgets.noninteractive.fg_stroke.color,
                opacity_tokens.hover,
            ),
            with_opacity(ui.visuals().selection.stroke.color, opacity_tokens.pressed),
            checked_progress,
        );
        let overlay_color = with_opacity(overlay_color, hover_progress);
        ui.painter()
            .add(Shape::rect_filled(rect, rect.height() / 2.0, overlay_color));
    }

    let geometry = geometry_for_rect_with_progress(rect, checked_progress);
    let stroke = Stroke::new(geometry.thickness, bar_color);
    let painter = ui.painter();
    painter.line_segment([geometry.top_start, geometry.top_end], stroke);
    painter.line_segment([geometry.middle_start, geometry.middle_end], stroke);
    painter.line_segment([geometry.bottom_start, geometry.bottom_end], stroke);

    response
}

#[must_use]
pub fn desired_size(size: Option<Vec2>) -> Vec2 {
    let minimum_size = minimum_button_size_token();
    let requested_size = size.unwrap_or(vec2(minimum_size, minimum_size));
    vec2(
        requested_size.x.max(minimum_size),
        requested_size.y.max(minimum_size),
    )
}

#[must_use]
pub fn next_checked_state(
    checked: bool,
    enabled: bool,
    toggle_on_click: bool,
    clicked: bool,
) -> bool {
    if enabled && clicked && toggle_on_click {
        !checked
    } else {
        checked
    }
}

#[must_use]
pub fn toggled_state_after_click(checked: bool, toggle_on_click: bool, clicked: bool) -> bool {
    next_checked_state(checked, true, toggle_on_click, clicked)
}

#[must_use]
pub fn geometry_for_rect(rect: Rect, checked: bool) -> HamburgerToggleButtonGeometry {
    let checked_progress = if checked { 1.0 } else { 0.0 };
    geometry_for_rect_with_progress(rect, checked_progress)
}

#[must_use]
pub fn geometry_for_rect_with_progress(
    rect: Rect,
    checked_progress: f32,
) -> HamburgerToggleButtonGeometry {
    let checked_progress = checked_progress.clamp(0.0, 1.0);
    let unchecked = unchecked_geometry_for_rect(rect);
    let checked = checked_geometry_for_rect(rect);

    HamburgerToggleButtonGeometry {
        top_start: lerp_pos2(unchecked.top_start, checked.top_start, checked_progress),
        top_end: lerp_pos2(unchecked.top_end, checked.top_end, checked_progress),
        middle_start: lerp_pos2(
            unchecked.middle_start,
            checked.middle_start,
            checked_progress,
        ),
        middle_end: lerp_pos2(unchecked.middle_end, checked.middle_end, checked_progress),
        bottom_start: lerp_pos2(
            unchecked.bottom_start,
            checked.bottom_start,
            checked_progress,
        ),
        bottom_end: lerp_pos2(unchecked.bottom_end, checked.bottom_end, checked_progress),
        thickness: egui::lerp(unchecked.thickness..=checked.thickness, checked_progress),
    }
}

fn unchecked_geometry_for_rect(rect: Rect) -> HamburgerToggleButtonGeometry {
    let content_padding = content_padding_token();
    let content_width_px = (rect.width() - content_padding * 2.0).max(0.0);
    let content_height_px = (rect.height() - content_padding * 2.0).max(0.0);

    let bar_thickness_px = (content_width_px.min(content_height_px) * 0.08).clamp(1.0, f32::MAX);
    let bar_inset_px = bar_thickness_px.max(1.0);
    let bar_spacing_px = ((content_height_px - 2.0 * bar_inset_px) / 4.0)
        .min(content_height_px * 0.2)
        .max(0.0);
    let bar_full_width_px = (content_width_px - bar_inset_px * 2.0).max(0.0);

    let start_x = rect.left() + content_padding + bar_inset_px;
    let top_y = rect.top() + content_padding + content_height_px / 2.0 - bar_spacing_px;
    let mid_y = rect.top() + content_padding + content_height_px / 2.0;
    let bottom_y = rect.top() + content_padding + content_height_px / 2.0 + bar_spacing_px;

    HamburgerToggleButtonGeometry {
        top_start: Pos2::new(start_x, top_y),
        top_end: Pos2::new(start_x + bar_full_width_px, top_y),
        middle_start: Pos2::new(start_x, mid_y),
        middle_end: Pos2::new(start_x + bar_full_width_px, mid_y),
        bottom_start: Pos2::new(start_x, bottom_y),
        bottom_end: Pos2::new(start_x + bar_full_width_px, bottom_y),
        thickness: bar_thickness_px,
    }
}

fn checked_geometry_for_rect(rect: Rect) -> HamburgerToggleButtonGeometry {
    let base = unchecked_geometry_for_rect(rect);
    let bar_full_width_px = base.middle_end.x - base.middle_start.x;
    let bar_half_width_px = bar_full_width_px / 2.0;
    let rotation = CHECKED_ROTATION_DEGREES.to_radians();
    let top_delta = vec2(
        bar_half_width_px * rotation.cos(),
        -bar_half_width_px * rotation.sin(),
    );
    let bottom_delta = vec2(
        bar_half_width_px * rotation.cos(),
        bar_half_width_px * rotation.sin(),
    );
    let mid_y = base.middle_start.y;
    let start_x = base.middle_start.x;

    HamburgerToggleButtonGeometry {
        top_start: Pos2::new(start_x, mid_y),
        top_end: Pos2::new(start_x + top_delta.x, mid_y + top_delta.y),
        middle_start: Pos2::new(start_x, mid_y),
        middle_end: Pos2::new(start_x + bar_full_width_px, mid_y),
        bottom_start: Pos2::new(start_x, mid_y),
        bottom_end: Pos2::new(start_x + bottom_delta.x, mid_y + bottom_delta.y),
        thickness: base.thickness,
    }
}

fn lerp_pos2(from: Pos2, to: Pos2, t: f32) -> Pos2 {
    Pos2::new(egui::lerp(from.x..=to.x, t), egui::lerp(from.y..=to.y, t))
}

fn lerp_color(from: Color32, to: Color32, t: f32) -> Color32 {
    let [fr, fg, fb, fa] = from.to_array();
    let [tr, tg, tb, ta] = to.to_array();

    let r = egui::lerp(f32::from(fr)..=f32::from(tr), t).round() as u8;
    let g = egui::lerp(f32::from(fg)..=f32::from(tg), t).round() as u8;
    let b = egui::lerp(f32::from(fb)..=f32::from(tb), t).round() as u8;
    let a = egui::lerp(f32::from(fa)..=f32::from(ta), t).round() as u8;

    Color32::from_rgba_unmultiplied(r, g, b, a)
}

fn with_opacity(color: Color32, alpha_factor: f32) -> Color32 {
    let [r, g, b, a] = color.to_array();
    let next_alpha = (f32::from(a) * alpha_factor).round().clamp(0.0, 255.0) as u8;
    Color32::from_rgba_unmultiplied(r, g, b, next_alpha)
}
