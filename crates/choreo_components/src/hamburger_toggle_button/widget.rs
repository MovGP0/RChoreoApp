use egui::Color32;
use egui::Response;
use egui::Sense;
use egui::Shape;
use egui::Stroke;
use egui::Ui;
use egui::Vec2;

use crate::material::styling::material_palette::MaterialPalette;
use crate::material::styling::material_palette::material_palette_for_visuals;

use super::geometry::desired_size;
use super::geometry::geometry_for_rect_with_progress;
use super::state::StateLayerVisualState;
use super::state::checked_animation_progress;
use super::state::next_checked_state;
use super::state::with_opacity;
use super::tokens::state_layer_animation_spec;
use super::tokens::state_opacity_tokens;

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

pub(super) fn draw_response(
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

    let palette = material_palette_for_visuals(ui.visuals());
    let checked_progress =
        checked_animation_progress(ui.ctx(), response.id.with("checked"), checked);
    let opacity_tokens = state_opacity_tokens();

    let mut bar_color = lerp_color(
        unchecked_bar_color(palette),
        checked_bar_color(palette),
        checked_progress,
    );

    if !enabled {
        bar_color = with_opacity(disabled_bar_color(palette), opacity_tokens.disabled);
    }

    let is_hovered_or_pressed =
        enabled && (response.hovered() || response.is_pointer_button_down_on());
    let hover_progress = state_layer_animation_spec().animate_bool(
        ui.ctx(),
        response.id.with("hover"),
        is_hovered_or_pressed,
    );

    if hover_progress > 0.0 {
        let overlay_fill = state_layer_fill(
            ui.visuals(),
            checked,
            StateLayerVisualState {
                interactive: enabled,
                hovered: response.hovered(),
                pressed: response.is_pointer_button_down_on(),
                focused: response.has_focus(),
            },
        );
        let overlay_color = with_opacity(overlay_fill, hover_progress);
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
fn unchecked_bar_color(palette: MaterialPalette) -> Color32 {
    palette.on_surface
}

#[must_use]
fn checked_bar_color(palette: MaterialPalette) -> Color32 {
    palette.secondary
}

#[must_use]
fn disabled_bar_color(palette: MaterialPalette) -> Color32 {
    palette.on_surface_variant
}

#[must_use]
fn state_layer_fill(
    visuals: &egui::Visuals,
    checked: bool,
    state: StateLayerVisualState,
) -> Color32 {
    let widget_visuals = if !state.interactive {
        visuals.widgets.noninteractive
    } else if state.pressed || state.focused {
        visuals.widgets.active
    } else if state.hovered {
        visuals.widgets.hovered
    } else {
        visuals.widgets.inactive
    };

    if checked {
        visuals.selection.bg_fill
    } else {
        widget_visuals.weak_bg_fill
    }
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

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use egui::Color32;

    use super::super::tokens::checked_animation_spec;
    use super::super::tokens::unchecked_animation_easing_spec;
    use super::super::tokens::unchecked_animation_spec;
    use super::MaterialPalette;
    use super::StateLayerVisualState;
    use super::checked_bar_color;
    use super::disabled_bar_color;
    use super::state_layer_fill;
    use crate::material::styling::material_animations::MaterialAnimation;
    use crate::material::styling::material_animations::MaterialAnimations;
    use crate::material::styling::material_palette::apply_material_visuals;
    use crate::material::styling::material_palette::material_palette_for_settings_state;
    use crate::settings::state::SettingsState;
    use crate::settings::state::ThemeMode;

    fn palette_fixture() -> MaterialPalette {
        let mut palette = MaterialPalette::light();
        palette.secondary = Color32::from_rgb(12, 34, 56);
        palette.on_surface = Color32::from_rgb(78, 90, 12);
        palette.on_surface_variant = Color32::from_rgb(120, 121, 122);
        palette
    }

    #[test]
    fn checked_hover_fill_uses_material_selection_background() {
        let context = egui::Context::default();
        let settings = SettingsState {
            theme_mode: ThemeMode::Dark,
            ..SettingsState::default()
        };
        apply_material_visuals(&context, &settings);
        let visuals = context.style().visuals.clone();
        let palette = material_palette_for_settings_state(&settings);

        assert_eq!(
            state_layer_fill(
                &visuals,
                true,
                StateLayerVisualState {
                    interactive: true,
                    hovered: true,
                    pressed: false,
                    focused: false,
                },
            ),
            palette.secondary_container
        );
    }

    #[test]
    fn unchecked_hover_fill_uses_hover_container_in_dark_mode() {
        let context = egui::Context::default();
        let settings = SettingsState {
            theme_mode: ThemeMode::Dark,
            ..SettingsState::default()
        };
        apply_material_visuals(&context, &settings);
        let visuals = context.style().visuals.clone();
        let palette = palette_fixture();

        assert_eq!(checked_bar_color(palette), palette.secondary);
        assert_eq!(
            state_layer_fill(
                &visuals,
                false,
                StateLayerVisualState {
                    interactive: true,
                    hovered: true,
                    pressed: false,
                    focused: false,
                },
            ),
            material_palette_for_settings_state(&settings).surface_container_high
        );
    }

    #[test]
    fn disabled_state_uses_surface_variant_role() {
        let palette = palette_fixture();

        assert_eq!(disabled_bar_color(palette), palette.on_surface_variant);
    }

    #[test]
    fn closing_animation_uses_faster_decelerating_curve() {
        assert_eq!(
            checked_animation_spec(),
            MaterialAnimations::spec(MaterialAnimation::Emphasized)
        );
        assert_eq!(
            unchecked_animation_easing_spec(),
            MaterialAnimations::spec(MaterialAnimation::StandardDecelerate)
        );
        assert_eq!(
            unchecked_animation_spec().duration,
            Duration::from_millis(150)
        );
        assert!(unchecked_animation_spec().duration < checked_animation_spec().duration);
    }
}
