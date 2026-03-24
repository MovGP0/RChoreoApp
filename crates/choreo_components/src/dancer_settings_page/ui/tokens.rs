use crate::material::styling::material_style_metrics::material_style_metrics;
use crate::material::styling::material_typography::TypographyRole;

// Kept for Slint parity during the egui migration.
const TOP_BAR_HEIGHT_PX: f32 = 64.0;
const CONTENT_MAX_WIDTH_PX: f32 = 720.0;
const CONTENT_OUTER_MARGIN_PX: f32 = 16.0;
const FOOTER_HEIGHT_PX: f32 = 56.0;
const FOOTER_PADDING_PX: f32 = 8.0;
const DROPDOWN_HEIGHT_PX: f32 = 60.0;

pub(super) const LIST_DRAWER_WIDTH_PX: f32 = 420.0;
pub(super) const SWAP_DANCERS_DIALOG_ID: &str = "swap_dancers";

#[must_use]
pub const fn content_spacing_token() -> f32 {
    material_style_metrics().spacings.spacing_12
}

#[must_use]
pub const fn card_corner_radius_token() -> f32 {
    material_style_metrics().corner_radii.border_radius_12
}

#[must_use]
pub const fn top_bar_height_token() -> f32 {
    TOP_BAR_HEIGHT_PX
}

#[must_use]
pub const fn content_max_width_token() -> f32 {
    CONTENT_MAX_WIDTH_PX
}

#[must_use]
pub const fn content_outer_margin_token() -> f32 {
    CONTENT_OUTER_MARGIN_PX
}

#[must_use]
pub const fn footer_height_token() -> f32 {
    FOOTER_HEIGHT_PX
}

#[must_use]
pub const fn footer_content_padding_token() -> f32 {
    FOOTER_PADDING_PX
}

#[must_use]
pub const fn footer_inner_height_token() -> f32 {
    footer_height_token() - (footer_content_padding_token() * 2.0)
}

#[must_use]
pub const fn dropdown_height_token() -> f32 {
    DROPDOWN_HEIGHT_PX
}

#[must_use]
pub const fn content_top_inset_token() -> f32 {
    content_outer_margin_token()
}

#[must_use]
pub const fn uses_scrollable_content_shell() -> bool {
    true
}

#[must_use]
pub const fn top_bar_title_role() -> TypographyRole {
    TypographyRole::TitleLarge
}
