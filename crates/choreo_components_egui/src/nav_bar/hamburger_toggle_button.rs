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

const DEFAULT_BUTTON_SIZE_PX: f32 = 48.0;
const CONTENT_PADDING_PX: f32 = 10.0;
const CHECKED_ROTATION_DEGREES: f32 = 35.0;
const DISABLED_OPACITY: f32 = 0.38;

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

#[must_use]
pub fn draw(
    ui: &mut Ui,
    checked: bool,
    enabled: bool,
    tooltip: &str,
    size: Option<Vec2>,
) -> Response {
    let desired_size = size.unwrap_or(vec2(DEFAULT_BUTTON_SIZE_PX, DEFAULT_BUTTON_SIZE_PX));
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

    let visuals = ui.style().interact(&response);
    let mut bar_color = if checked {
        ui.visuals().selection.stroke.color
    } else {
        visuals.fg_stroke.color
    };

    if !enabled {
        bar_color = with_opacity(bar_color, DISABLED_OPACITY);
    }

    if response.hovered() || response.is_pointer_button_down_on() {
        let overlay_color = if checked {
            with_opacity(ui.visuals().selection.stroke.color, 0.16)
        } else {
            with_opacity(ui.visuals().widgets.noninteractive.fg_stroke.color, 0.10)
        };
        ui.painter()
            .add(Shape::rect_filled(rect, rect.height() / 2.0, overlay_color));
    }

    let geometry = geometry_for_rect(rect, checked);
    let stroke = Stroke::new(geometry.thickness, bar_color);
    let painter = ui.painter();
    painter.line_segment([geometry.top_start, geometry.top_end], stroke);
    painter.line_segment([geometry.middle_start, geometry.middle_end], stroke);
    painter.line_segment([geometry.bottom_start, geometry.bottom_end], stroke);

    response
}

#[must_use]
pub fn geometry_for_rect(rect: Rect, checked: bool) -> HamburgerToggleButtonGeometry {
    let content_width_px = (rect.width() - CONTENT_PADDING_PX * 2.0).max(0.0);
    let content_height_px = (rect.height() - CONTENT_PADDING_PX * 2.0).max(0.0);

    let bar_thickness_px = (content_width_px.min(content_height_px) * 0.08).clamp(1.0, f32::MAX);
    let bar_inset_px = bar_thickness_px.max(1.0);
    let bar_spacing_px = ((content_height_px - 2.0 * bar_inset_px) / 4.0)
        .min(content_height_px * 0.2)
        .max(0.0);
    let bar_full_width_px = (content_width_px - bar_inset_px * 2.0).max(0.0);
    let bar_half_width_px = bar_full_width_px / 2.0;

    let start_x = rect.left() + CONTENT_PADDING_PX + bar_inset_px;
    let top_y = rect.top() + CONTENT_PADDING_PX + content_height_px / 2.0 - bar_spacing_px;
    let mid_y = rect.top() + CONTENT_PADDING_PX + content_height_px / 2.0;
    let bottom_y = rect.top() + CONTENT_PADDING_PX + content_height_px / 2.0 + bar_spacing_px;

    if checked {
        let rotation = CHECKED_ROTATION_DEGREES.to_radians();
        let top_delta = vec2(
            bar_half_width_px * rotation.cos(),
            -bar_half_width_px * rotation.sin(),
        );
        let bottom_delta = vec2(
            bar_half_width_px * rotation.cos(),
            bar_half_width_px * rotation.sin(),
        );

        HamburgerToggleButtonGeometry {
            top_start: Pos2::new(start_x, mid_y),
            top_end: Pos2::new(start_x + top_delta.x, mid_y + top_delta.y),
            middle_start: Pos2::new(start_x, mid_y),
            middle_end: Pos2::new(start_x + bar_full_width_px, mid_y),
            bottom_start: Pos2::new(start_x, mid_y),
            bottom_end: Pos2::new(start_x + bottom_delta.x, mid_y + bottom_delta.y),
            thickness: bar_thickness_px,
        }
    } else {
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
}

fn with_opacity(color: Color32, alpha_factor: f32) -> Color32 {
    let [r, g, b, a] = color.to_array();
    let next_alpha = (f32::from(a) * alpha_factor).round().clamp(0.0, 255.0) as u8;
    Color32::from_rgba_unmultiplied(r, g, b, next_alpha)
}
