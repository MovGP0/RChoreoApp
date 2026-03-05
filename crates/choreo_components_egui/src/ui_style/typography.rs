use egui::Context;
use egui::FontFamily;
use egui::FontId;
use egui::RichText;
use egui::Style;
use egui::TextStyle;

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
pub struct TypographyStyle {
    pub font_size_px: f32,
    pub line_height_px: f32,
    pub font_weight: i32,
}

const FONT_WEIGHT_REGULAR: i32 = 300;
const FONT_WEIGHT_MEDIUM: i32 = 600;
const FONT_WEIGHT_SEMIBOLD: i32 = 900;

pub const fn style_for_role(role: TypographyRole) -> TypographyStyle {
    match role {
        TypographyRole::DisplayLarge => TypographyStyle {
            font_size_px: 57.0,
            line_height_px: 64.0,
            font_weight: FONT_WEIGHT_REGULAR,
        },
        TypographyRole::DisplayMedium => TypographyStyle {
            font_size_px: 45.0,
            line_height_px: 52.0,
            font_weight: FONT_WEIGHT_REGULAR,
        },
        TypographyRole::DisplaySmall => TypographyStyle {
            font_size_px: 36.0,
            line_height_px: 44.0,
            font_weight: FONT_WEIGHT_REGULAR,
        },
        TypographyRole::HeadlineLarge => TypographyStyle {
            font_size_px: 32.0,
            line_height_px: 40.0,
            font_weight: FONT_WEIGHT_REGULAR,
        },
        TypographyRole::HeadlineMedium => TypographyStyle {
            font_size_px: 28.0,
            line_height_px: 36.0,
            font_weight: FONT_WEIGHT_REGULAR,
        },
        TypographyRole::HeadlineSmall => TypographyStyle {
            font_size_px: 24.0,
            line_height_px: 32.0,
            font_weight: FONT_WEIGHT_REGULAR,
        },
        TypographyRole::TitleLarge => TypographyStyle {
            font_size_px: 22.0,
            line_height_px: 28.0,
            font_weight: FONT_WEIGHT_REGULAR,
        },
        TypographyRole::TitleMedium => TypographyStyle {
            font_size_px: 16.0,
            line_height_px: 24.0,
            font_weight: FONT_WEIGHT_MEDIUM,
        },
        TypographyRole::TitleSmall => TypographyStyle {
            font_size_px: 14.0,
            line_height_px: 20.0,
            font_weight: FONT_WEIGHT_MEDIUM,
        },
        TypographyRole::LabelLarge => TypographyStyle {
            font_size_px: 14.0,
            line_height_px: 20.0,
            font_weight: FONT_WEIGHT_MEDIUM,
        },
        TypographyRole::LabelMedium => TypographyStyle {
            font_size_px: 12.0,
            line_height_px: 16.0,
            font_weight: FONT_WEIGHT_MEDIUM,
        },
        TypographyRole::LabelMediumProminent => TypographyStyle {
            font_size_px: 12.0,
            line_height_px: 16.0,
            font_weight: FONT_WEIGHT_SEMIBOLD,
        },
        TypographyRole::LabelSmall => TypographyStyle {
            font_size_px: 11.0,
            line_height_px: 16.0,
            font_weight: FONT_WEIGHT_MEDIUM,
        },
        TypographyRole::BodyLarge => TypographyStyle {
            font_size_px: 16.0,
            line_height_px: 24.0,
            font_weight: FONT_WEIGHT_REGULAR,
        },
        TypographyRole::BodyMedium => TypographyStyle {
            font_size_px: 14.0,
            line_height_px: 20.0,
            font_weight: FONT_WEIGHT_REGULAR,
        },
        TypographyRole::BodySmall => TypographyStyle {
            font_size_px: 12.0,
            line_height_px: 16.0,
            font_weight: FONT_WEIGHT_REGULAR,
        },
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
    FontId::new(style_for_role(role).font_size_px, FontFamily::Proportional)
}

#[must_use]
pub fn rich_text_for_role(text: impl Into<String>, role: TypographyRole) -> RichText {
    let style = style_for_role(role);
    let rich_text = RichText::new(text).font(font_id_for_role(role));
    if style.font_weight >= FONT_WEIGHT_MEDIUM {
        return rich_text.strong();
    }
    rich_text
}

#[must_use]
pub fn apply_text_styles(style: &Style) -> Style {
    let mut updated = style.clone();
    updated.text_styles = [
        (
            TextStyle::Heading,
            font_id_for_role(TypographyRole::HeadlineSmall),
        ),
        (
            TextStyle::Body,
            font_id_for_role(TypographyRole::BodyMedium),
        ),
        (
            TextStyle::Button,
            font_id_for_role(TypographyRole::LabelLarge),
        ),
        (
            TextStyle::Small,
            font_id_for_role(TypographyRole::BodySmall),
        ),
        (
            TextStyle::Monospace,
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
