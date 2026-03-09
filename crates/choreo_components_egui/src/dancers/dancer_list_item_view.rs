use choreo_master_mobile_json::Color;
use egui::Align2;
use egui::Color32;
use egui::Pos2;
use egui::Rect;
use egui::Response;
use egui::Sense;
use egui::Ui;
use egui::Vec2;
use egui::pos2;
use egui::vec2;

use crate::material::styling::material_typography as typography;
use crate::material::styling::material_typography::TypographyRole;

use super::state::DancerState;

const ITEM_TOP_BOTTOM_GAP_PX: f32 = 3.0;
const ITEM_CORNER_RADIUS_PX: f32 = 8.0;
const SWATCH_X_PX: f32 = 10.0;
const SWATCH_HALF_HEIGHT_PX: f32 = 14.0;
const SWATCH_SIZE_PX: f32 = 28.0;
const SWATCH_CORNER_RADIUS_PX: f32 = 6.0;
const TITLE_X_PX: f32 = 46.0;
const TITLE_Y_PX: f32 = 8.0;
const SUBTITLE_Y_PX: f32 = 28.0;
pub const ROW_HEIGHT_PX: f32 = 56.0;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DancerListItemLayout {
    pub content_rect: Rect,
    pub swatch_rect: Rect,
    pub title_position: Pos2,
    pub subtitle_position: Pos2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DancerListItemColors {
    pub background: Color32,
    pub border: Color32,
    pub title: Color32,
    pub subtitle: Color32,
}

#[must_use]
pub fn draw(ui: &mut Ui, dancer: &DancerState, is_selected: bool) -> Response {
    let row_size = vec2(ui.available_width(), ROW_HEIGHT_PX);
    let (row_rect, response) = ui.allocate_exact_size(row_size, Sense::click());
    if !ui.is_rect_visible(row_rect) {
        return response;
    }

    let layout = layout_for_row_rect(row_rect);
    let colors = colors_for_selection(&ui.style().visuals, is_selected);

    ui.painter().rect(
        layout.content_rect,
        ITEM_CORNER_RADIUS_PX,
        colors.background,
        egui::Stroke::new(1.0, colors.border),
        egui::StrokeKind::Middle,
    );
    ui.painter().rect_filled(
        layout.swatch_rect,
        SWATCH_CORNER_RADIUS_PX,
        color_to_egui(&dancer.color),
    );
    ui.painter().text(
        layout.title_position,
        Align2::LEFT_TOP,
        dancer.name.as_str(),
        typography::font_id_for_role(title_role()),
        colors.title,
    );
    ui.painter().text(
        layout.subtitle_position,
        Align2::LEFT_TOP,
        supporting_text(dancer),
        typography::font_id_for_role(subtitle_role()),
        colors.subtitle,
    );

    response
}

#[must_use]
pub const fn title_role() -> TypographyRole {
    TypographyRole::BodyMedium
}

#[must_use]
pub const fn subtitle_role() -> TypographyRole {
    TypographyRole::BodySmall
}

#[must_use]
pub fn layout_for_row_rect(row_rect: Rect) -> DancerListItemLayout {
    let content_rect = row_rect.shrink2(vec2(0.0, ITEM_TOP_BOTTOM_GAP_PX));
    let swatch_rect = Rect::from_min_size(
        pos2(
            content_rect.left() + SWATCH_X_PX,
            content_rect.center().y - SWATCH_HALF_HEIGHT_PX,
        ),
        vec2(SWATCH_SIZE_PX, SWATCH_SIZE_PX),
    );
    let text_left = content_rect.left() + TITLE_X_PX;

    DancerListItemLayout {
        content_rect,
        swatch_rect,
        title_position: pos2(text_left, content_rect.top() + TITLE_Y_PX),
        subtitle_position: pos2(text_left, content_rect.top() + SUBTITLE_Y_PX),
    }
}

#[must_use]
pub fn colors_for_selection(visuals: &egui::Visuals, is_selected: bool) -> DancerListItemColors {
    let (background, border, title) = if is_selected {
        (
            visuals.selection.bg_fill,
            visuals.selection.stroke.color,
            visuals.strong_text_color(),
        )
    } else {
        (
            visuals.extreme_bg_color,
            visuals.widgets.noninteractive.bg_stroke.color,
            visuals.text_color(),
        )
    };

    DancerListItemColors {
        background,
        border,
        title,
        subtitle: visuals.weak_text_color(),
    }
}

#[must_use]
pub fn supporting_text(dancer: &DancerState) -> String {
    format!(
        "{} ({})  [{}]",
        dancer.role.name, dancer.role.z_index, dancer.shortcut
    )
}

#[must_use]
pub fn role_details_text(dancer: &DancerState) -> String {
    supporting_text(dancer)
}

#[must_use]
pub fn row_size(available_width: f32) -> Vec2 {
    vec2(available_width, ROW_HEIGHT_PX)
}

fn color_to_egui(color: &Color) -> Color32 {
    Color32::from_rgba_unmultiplied(color.r, color.g, color.b, color.a)
}
