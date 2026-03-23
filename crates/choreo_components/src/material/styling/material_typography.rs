use egui::Context;
use egui::FontFamily;
use egui::FontId;
use egui::RichText;
use egui::Style;
use egui::TextStyle as EguiTextStyle;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypographyRole {
    DisplayLarge,
    DisplayMedium,
    DisplaySmall,
    HeadlineLarge,
    HeadlineMedium,
    HeadlineSmall,
    TitleLarge,
    TitleMedium,
    TitleSmall,
    LabelLarge,
    LabelMedium,
    LabelMediumProminent,
    LabelSmall,
    BodyLarge,
    BodyMedium,
    BodySmall,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TextStyle {
    pub font_size_px: f32,
    pub font_weight: i32,
}

impl TextStyle {
    #[must_use]
    pub const fn new(font_size_px: f32, font_weight: i32) -> Self {
        Self {
            font_size_px,
            font_weight,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MaterialTypography {
    pub regular: i32,
    pub medium: i32,
    pub semibold: i32,
    pub display_large: TextStyle,
    pub display_medium: TextStyle,
    pub display_small: TextStyle,
    pub headline_large: TextStyle,
    pub headline_medium: TextStyle,
    pub headline_small: TextStyle,
    pub title_large: TextStyle,
    pub title_medium: TextStyle,
    pub title_small: TextStyle,
    pub label_large: TextStyle,
    pub label_medium: TextStyle,
    pub label_medium_prominent: TextStyle,
    pub label_small: TextStyle,
    pub body_large: TextStyle,
    pub body_medium: TextStyle,
    pub body_small: TextStyle,
}

impl MaterialTypography {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            regular: FONT_WEIGHT_REGULAR,
            medium: FONT_WEIGHT_MEDIUM,
            semibold: FONT_WEIGHT_SEMIBOLD,
            display_large: TextStyle::new(57.0, FONT_WEIGHT_REGULAR),
            display_medium: TextStyle::new(45.0, FONT_WEIGHT_REGULAR),
            display_small: TextStyle::new(36.0, FONT_WEIGHT_REGULAR),
            headline_large: TextStyle::new(32.0, FONT_WEIGHT_REGULAR),
            headline_medium: TextStyle::new(28.0, FONT_WEIGHT_REGULAR),
            headline_small: TextStyle::new(24.0, FONT_WEIGHT_REGULAR),
            title_large: TextStyle::new(22.0, FONT_WEIGHT_REGULAR),
            title_medium: TextStyle::new(16.0, FONT_WEIGHT_MEDIUM),
            title_small: TextStyle::new(14.0, FONT_WEIGHT_MEDIUM),
            label_large: TextStyle::new(14.0, FONT_WEIGHT_MEDIUM),
            label_medium: TextStyle::new(12.0, FONT_WEIGHT_MEDIUM),
            label_medium_prominent: TextStyle::new(12.0, FONT_WEIGHT_SEMIBOLD),
            label_small: TextStyle::new(11.0, FONT_WEIGHT_MEDIUM),
            body_large: TextStyle::new(16.0, FONT_WEIGHT_REGULAR),
            body_medium: TextStyle::new(14.0, FONT_WEIGHT_REGULAR),
            body_small: TextStyle::new(12.0, FONT_WEIGHT_REGULAR),
        }
    }

    #[must_use]
    pub const fn text_style_for_role(self, role: TypographyRole) -> TextStyle {
        match role {
            TypographyRole::DisplayLarge => self.display_large,
            TypographyRole::DisplayMedium => self.display_medium,
            TypographyRole::DisplaySmall => self.display_small,
            TypographyRole::HeadlineLarge => self.headline_large,
            TypographyRole::HeadlineMedium => self.headline_medium,
            TypographyRole::HeadlineSmall => self.headline_small,
            TypographyRole::TitleLarge => self.title_large,
            TypographyRole::TitleMedium => self.title_medium,
            TypographyRole::TitleSmall => self.title_small,
            TypographyRole::LabelLarge => self.label_large,
            TypographyRole::LabelMedium => self.label_medium,
            TypographyRole::LabelMediumProminent => self.label_medium_prominent,
            TypographyRole::LabelSmall => self.label_small,
            TypographyRole::BodyLarge => self.body_large,
            TypographyRole::BodyMedium => self.body_medium,
            TypographyRole::BodySmall => self.body_small,
        }
    }
}

impl Default for MaterialTypography {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TypographyStyle {
    pub font_size_px: f32,
    pub line_height_px: f32,
    pub font_weight: i32,
}

impl TypographyStyle {
    #[must_use]
    pub const fn new(font_size_px: f32, line_height_px: f32, font_weight: i32) -> Self {
        Self {
            font_size_px,
            line_height_px,
            font_weight,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontWeightEmphasis {
    Regular,
    Medium,
    Semibold,
}

pub const FONT_WEIGHT_REGULAR: i32 = 300;
pub const FONT_WEIGHT_MEDIUM: i32 = 600;
pub const FONT_WEIGHT_SEMIBOLD: i32 = 900;

pub const MATERIAL_TYPOGRAPHY: MaterialTypography = MaterialTypography::new();

#[must_use]
pub const fn text_style_for_role(role: TypographyRole) -> TextStyle {
    MATERIAL_TYPOGRAPHY.text_style_for_role(role)
}

#[must_use]
pub const fn style_for_role(role: TypographyRole) -> TypographyStyle {
    let text_style = text_style_for_role(role);
    TypographyStyle::new(
        text_style.font_size_px,
        line_height_for_role(role),
        text_style.font_weight,
    )
}

#[must_use]
pub const fn line_height_for_role(role: TypographyRole) -> f32 {
    match role {
        TypographyRole::DisplayLarge => 64.0,
        TypographyRole::DisplayMedium => 52.0,
        TypographyRole::DisplaySmall => 44.0,
        TypographyRole::HeadlineLarge => 40.0,
        TypographyRole::HeadlineMedium => 36.0,
        TypographyRole::HeadlineSmall => 32.0,
        TypographyRole::TitleLarge => 28.0,
        TypographyRole::TitleMedium => 24.0,
        TypographyRole::TitleSmall => 20.0,
        TypographyRole::LabelLarge => 20.0,
        TypographyRole::LabelMedium => 16.0,
        TypographyRole::LabelMediumProminent => 16.0,
        TypographyRole::LabelSmall => 16.0,
        TypographyRole::BodyLarge => 24.0,
        TypographyRole::BodyMedium => 20.0,
        TypographyRole::BodySmall => 16.0,
    }
}

#[must_use]
pub fn platform_font_fallback_chain() -> &'static [&'static str] {
    &[
        "Roboto",
        "Segoe UI",
        "Noto Sans",
        "Helvetica Neue",
        "Arial",
        "sans-serif",
    ]
}

#[must_use]
pub fn font_id_for_role(role: TypographyRole) -> FontId {
    FontId::new(
        text_style_for_role(role).font_size_px,
        FontFamily::Proportional,
    )
}

#[must_use]
pub fn rich_text_for_role(text: impl Into<String>, role: TypographyRole) -> RichText {
    let text_style = text_style_for_role(role);
    apply_font_weight_to_rich_text(RichText::new(text).font(font_id_for_role(role)), text_style)
}

#[must_use]
pub const fn font_weight_emphasis(font_weight: i32) -> FontWeightEmphasis {
    if font_weight >= FONT_WEIGHT_SEMIBOLD {
        FontWeightEmphasis::Semibold
    } else if font_weight >= FONT_WEIGHT_MEDIUM {
        FontWeightEmphasis::Medium
    } else {
        FontWeightEmphasis::Regular
    }
}

#[must_use]
pub fn apply_font_weight_to_rich_text(rich_text: RichText, style: TextStyle) -> RichText {
    match font_weight_emphasis(style.font_weight) {
        FontWeightEmphasis::Regular => rich_text,
        FontWeightEmphasis::Medium => rich_text.extra_letter_spacing(0.2),
        FontWeightEmphasis::Semibold => rich_text.strong(),
    }
}

#[must_use]
pub fn apply_text_styles(style: &Style) -> Style {
    let mut updated = style.clone();
    updated.text_styles = [
        (
            EguiTextStyle::Heading,
            font_id_for_role(TypographyRole::HeadlineSmall),
        ),
        (
            EguiTextStyle::Body,
            font_id_for_role(TypographyRole::BodyMedium),
        ),
        (
            EguiTextStyle::Button,
            font_id_for_role(TypographyRole::LabelLarge),
        ),
        (
            EguiTextStyle::Small,
            font_id_for_role(TypographyRole::BodySmall),
        ),
        (
            EguiTextStyle::Monospace,
            FontId::new(12.0, FontFamily::Monospace),
        ),
    ]
    .into();
    updated
}

pub fn apply_to_context(context: &Context) {
    let style = apply_text_styles(context.style().as_ref());
    context.set_style(style);
}

#[cfg(test)]
mod tests {
    use super::FONT_WEIGHT_MEDIUM;
    use super::FONT_WEIGHT_REGULAR;
    use super::FONT_WEIGHT_SEMIBOLD;
    use super::FontWeightEmphasis;
    use super::TextStyle;
    use super::apply_font_weight_to_rich_text;
    use super::font_weight_emphasis;

    #[test]
    fn medium_weight_maps_to_distinct_emphasis() {
        assert_eq!(
            font_weight_emphasis(FONT_WEIGHT_REGULAR),
            FontWeightEmphasis::Regular
        );
        assert_eq!(
            font_weight_emphasis(FONT_WEIGHT_MEDIUM),
            FontWeightEmphasis::Medium
        );
        assert_eq!(
            font_weight_emphasis(FONT_WEIGHT_SEMIBOLD),
            FontWeightEmphasis::Semibold
        );
    }

    #[test]
    fn medium_weight_text_still_builds_rich_text() {
        let style = TextStyle::new(14.0, FONT_WEIGHT_MEDIUM);
        let _ = apply_font_weight_to_rich_text(egui::RichText::new("medium"), style);
    }
}
