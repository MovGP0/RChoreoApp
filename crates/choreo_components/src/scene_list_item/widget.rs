use egui::Align2;
use egui::Color32;
use egui::CornerRadius;
use egui::Response;
use egui::Sense;
use egui::Stroke;
use egui::Ui;
use egui::vec2;

use crate::material::styling::material_palette::MaterialPalette;
use crate::material::styling::material_palette::material_palette_for_visuals;
use crate::material::styling::material_style_metrics::material_style_metrics;
use crate::material::styling::material_typography as typography;
use crate::material::styling::material_typography::TypographyRole;
use crate::scene_list_item::geometry::layout_for_row_rect;
use crate::scene_list_item::geometry::row_height_px;
use crate::scene_list_item::state::SceneItemState;
use crate::time::format_seconds;

const SCENE_ROW_SWATCH_CORNER_RADIUS_PX: u8 = 4;

#[must_use]
pub const fn title_role() -> TypographyRole {
    TypographyRole::BodyMedium
}

#[must_use]
pub const fn timestamp_role() -> TypographyRole {
    TypographyRole::LabelMedium
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SceneListItemColors {
    pub background: Color32,
    pub border: Color32,
    pub title: Color32,
    pub timestamp: Color32,
    pub accent: Color32,
    pub border_width: f32,
}

#[must_use]
pub fn colors_for_selection(palette: MaterialPalette, is_selected: bool) -> SceneListItemColors {
    let metrics = material_style_metrics();
    if is_selected {
        SceneListItemColors {
            background: palette.surface_container_high,
            border: palette.secondary,
            title: palette.on_surface,
            timestamp: palette.on_surface_variant,
            accent: palette.secondary,
            border_width: metrics.strokes.focus,
        }
    } else {
        SceneListItemColors {
            background: palette.surface_container_low,
            border: palette.outline_variant,
            title: palette.on_surface,
            timestamp: palette.on_surface_variant,
            accent: palette.outline_variant,
            border_width: metrics.strokes.outline,
        }
    }
}

#[must_use]
pub fn draw(ui: &mut Ui, scene: &SceneItemState, show_timestamps: bool) -> Response {
    let (row_rect, response) = ui.allocate_exact_size(
        vec2(ui.available_width(), row_height_px(show_timestamps)),
        Sense::click(),
    );
    if !ui.is_rect_visible(row_rect) {
        return response;
    }

    let layout = layout_for_row_rect(row_rect, show_timestamps);
    let palette = material_palette_for_visuals(&ui.style().visuals);
    let colors = colors_for_selection(palette, scene.is_selected);

    ui.painter().rect(
        layout.content_rect,
        CornerRadius::same(material_style_metrics().sizes.size_6 as u8),
        colors.background,
        Stroke::new(colors.border_width, colors.border),
        egui::StrokeKind::Middle,
    );

    if scene.is_selected {
        ui.painter().rect_filled(
            layout.accent_rect,
            CornerRadius::same(material_style_metrics().sizes.size_6 as u8),
            colors.accent,
        );
    }

    ui.painter().rect_filled(
        layout.swatch_rect,
        CornerRadius::same(SCENE_ROW_SWATCH_CORNER_RADIUS_PX),
        Color32::from_rgba_unmultiplied(scene.color.r, scene.color.g, scene.color.b, scene.color.a),
    );

    ui.painter().text(
        layout.title_position,
        Align2::LEFT_TOP,
        scene.name.as_str(),
        typography::font_id_for_role(title_role()),
        colors.title,
    );

    if show_timestamps {
        let timestamp_text = scene.timestamp.map(format_seconds).unwrap_or_default();
        ui.painter().text(
            layout.timestamp_position,
            Align2::LEFT_TOP,
            timestamp_text,
            typography::font_id_for_role(timestamp_role()),
            colors.timestamp,
        );
    }

    response
}
