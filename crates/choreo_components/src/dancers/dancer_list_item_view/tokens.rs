use crate::material::styling::material_typography::TypographyRole;

pub const ROW_HEIGHT_PX: f32 = 56.0;

const ITEM_TOP_BOTTOM_GAP_PX: f32 = 3.0;
const ITEM_CORNER_RADIUS_PX: f32 = 8.0;
const SWATCH_X_PX: f32 = 10.0;
const SWATCH_HALF_HEIGHT_PX: f32 = 14.0;
const SWATCH_SIZE_PX: f32 = 28.0;
const SWATCH_CORNER_RADIUS_PX: f32 = 6.0;
const TITLE_X_PX: f32 = 46.0;
const TITLE_Y_PX: f32 = 8.0;
const SUBTITLE_Y_PX: f32 = 28.0;

#[must_use]
pub const fn title_role() -> TypographyRole {
    TypographyRole::BodyMedium
}

#[must_use]
pub const fn subtitle_role() -> TypographyRole {
    TypographyRole::BodySmall
}

#[must_use]
pub(super) const fn item_top_bottom_gap_token() -> f32 {
    ITEM_TOP_BOTTOM_GAP_PX
}

#[must_use]
pub(super) const fn item_corner_radius_token() -> f32 {
    ITEM_CORNER_RADIUS_PX
}

#[must_use]
pub(super) const fn swatch_x_token() -> f32 {
    SWATCH_X_PX
}

#[must_use]
pub(super) const fn swatch_half_height_token() -> f32 {
    SWATCH_HALF_HEIGHT_PX
}

#[must_use]
pub(super) const fn swatch_size_token() -> f32 {
    SWATCH_SIZE_PX
}

#[must_use]
pub(super) const fn swatch_corner_radius_token() -> f32 {
    SWATCH_CORNER_RADIUS_PX
}

#[must_use]
pub(super) const fn title_x_token() -> f32 {
    TITLE_X_PX
}

#[must_use]
pub(super) const fn title_y_token() -> f32 {
    TITLE_Y_PX
}

#[must_use]
pub(super) const fn subtitle_y_token() -> f32 {
    SUBTITLE_Y_PX
}
