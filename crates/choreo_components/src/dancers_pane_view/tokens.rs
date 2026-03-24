use crate::material::styling::material_style_metrics::material_style_metrics;

#[must_use]
pub const fn pane_spacing_token() -> f32 {
    material_style_metrics().spacings.spacing_12
}

#[must_use]
pub const fn pane_corner_radius_token() -> f32 {
    material_style_metrics().corner_radii.border_radius_12
}

#[must_use]
pub const fn pane_inner_padding_token() -> i8 {
    material_style_metrics().paddings.padding_8 as i8
}
