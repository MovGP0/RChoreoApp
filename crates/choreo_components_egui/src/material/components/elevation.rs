use egui::Color32;
use egui::CornerRadius;
use egui::Painter;
use egui::Rect;
use egui::vec2;

use crate::material::styling::material_palette::MaterialPalette;
use crate::material::styling::material_palette::material_palette_for_visuals;
use crate::material::styling::material_style_metrics::ElevationShadow;
use crate::material::styling::material_style_metrics::material_style_metrics;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ElevationSpec {
    pub background: Color32,
    pub border_radius: f32,
    pub level: u8,
    pub dark_mode: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ElevationShadowSpec {
    pub offset_y: f32,
    pub blur: f32,
    pub color: Color32,
}

#[must_use]
pub fn elevation_spec(background: Color32, border_radius: f32, level: u8, dark_mode: bool) -> ElevationSpec {
    ElevationSpec {
        background,
        border_radius,
        level: level.clamp(1, 5),
        dark_mode,
    }
}

#[must_use]
pub fn elevation_shadows(spec: ElevationSpec, palette: MaterialPalette) -> [ElevationShadowSpec; 2] {
    let tokens = material_style_metrics();
    let (outer, inner, outer_color, inner_color) = match (spec.dark_mode, spec.level) {
        (false, 1) => (
            tokens.elevations.level_1.outer,
            tokens.elevations.level_1.inner,
            palette.shadow_30,
            palette.shadow_15,
        ),
        (false, 2) => (
            tokens.elevations.level_2.outer,
            tokens.elevations.level_2.inner,
            palette.shadow_30,
            palette.shadow_15,
        ),
        (false, 3) => (
            tokens.elevations.level_3.outer,
            tokens.elevations.level_3.inner,
            palette.shadow_15,
            palette.shadow_30,
        ),
        (false, 4) => (
            tokens.elevations.level_4.outer,
            tokens.elevations.level_4.inner,
            palette.shadow_15,
            palette.shadow_30,
        ),
        (false, _) => (
            tokens.elevations.level_5.outer,
            tokens.elevations.level_5.inner,
            palette.shadow_15,
            palette.shadow_30,
        ),
        (true, 1) => (
            ElevationShadow {
                offset_y: 1.0,
                blur: 3.0,
                opacity: 0.15,
            },
            ElevationShadow {
                offset_y: 1.0,
                blur: 2.0,
                opacity: 0.30,
            },
            palette.shadow_15,
            palette.shadow_30,
        ),
        (true, 2) => (
            ElevationShadow {
                offset_y: 2.0,
                blur: 6.0,
                opacity: 0.15,
            },
            ElevationShadow {
                offset_y: 1.0,
                blur: 2.0,
                opacity: 0.30,
            },
            palette.shadow_15,
            palette.shadow_30,
        ),
        (true, 3) => (
            tokens.elevations.level_3.outer,
            tokens.elevations.level_3.inner,
            palette.shadow_15,
            palette.shadow_30,
        ),
        (true, 4) => (
            tokens.elevations.level_4.outer,
            tokens.elevations.level_4.inner,
            palette.shadow_15,
            palette.shadow_30,
        ),
        (true, _) => (
            tokens.elevations.level_5.outer,
            tokens.elevations.level_5.inner,
            palette.shadow_15,
            palette.shadow_30,
        ),
    };
    [shadow_spec(outer, outer_color), shadow_spec(inner, inner_color)]
}

pub fn paint_elevation(painter: &Painter, rect: Rect, spec: ElevationSpec, palette: MaterialPalette) {
    let rounding = CornerRadius::same(spec.border_radius.round() as u8);
    for shadow in elevation_shadows(spec, palette) {
        let shadow_rect = rect
            .translate(vec2(0.0, shadow.offset_y))
            .expand(shadow.blur * 0.5);
        painter.rect_filled(shadow_rect, rounding, shadow.color);
    }
    painter.rect_filled(rect, rounding, spec.background);
}

pub fn paint_elevation_for_ui(painter: &Painter, rect: Rect, spec: ElevationSpec, visuals: &egui::Visuals) {
    paint_elevation(
        painter,
        rect,
        spec,
        material_palette_for_visuals(visuals),
    );
}

const fn shadow_spec(shadow: ElevationShadow, color: Color32) -> ElevationShadowSpec {
    ElevationShadowSpec {
        offset_y: shadow.offset_y,
        blur: shadow.blur,
        color,
    }
}

#[cfg(test)]
mod tests {
    use egui::Color32;

    use super::elevation_shadows;
    use super::elevation_spec;
    use crate::material::styling::material_palette::MaterialPalette;

    #[test]
    fn dark_mode_swaps_shadow_opacity_order() {
        let palette = MaterialPalette::dark();
        let shadows = elevation_shadows(elevation_spec(Color32::WHITE, 12.0, 1, true), palette);
        assert_eq!(shadows[0].color, palette.shadow_15);
        assert_eq!(shadows[1].color, palette.shadow_30);
        assert_eq!(shadows[0].blur, 3.0);
        assert_eq!(shadows[1].blur, 2.0);
    }
}
