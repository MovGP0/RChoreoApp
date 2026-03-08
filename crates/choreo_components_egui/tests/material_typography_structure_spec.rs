use choreo_components_egui::material::styling::material_typography::FONT_WEIGHT_MEDIUM;
use choreo_components_egui::material::styling::material_typography::FONT_WEIGHT_REGULAR;
use choreo_components_egui::material::styling::material_typography::FONT_WEIGHT_SEMIBOLD;
use choreo_components_egui::material::styling::material_typography::MATERIAL_TYPOGRAPHY;
use choreo_components_egui::material::styling::material_typography::MaterialTypography;
use choreo_components_egui::material::styling::material_typography::TextStyle;
use choreo_components_egui::material::styling::material_typography::TypographyRole;
use choreo_components_egui::material::styling::material_typography::text_style_for_role;

#[test]
fn material_typography_matches_slint_global_structure() {
    assert_eq!(
        MATERIAL_TYPOGRAPHY,
        MaterialTypography {
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
    );
}

#[test]
fn typography_role_lookup_reads_from_material_typography() {
    for (role, expected) in [
        (
            TypographyRole::DisplayLarge,
            MATERIAL_TYPOGRAPHY.display_large,
        ),
        (
            TypographyRole::DisplayMedium,
            MATERIAL_TYPOGRAPHY.display_medium,
        ),
        (
            TypographyRole::DisplaySmall,
            MATERIAL_TYPOGRAPHY.display_small,
        ),
        (
            TypographyRole::HeadlineLarge,
            MATERIAL_TYPOGRAPHY.headline_large,
        ),
        (
            TypographyRole::HeadlineMedium,
            MATERIAL_TYPOGRAPHY.headline_medium,
        ),
        (
            TypographyRole::HeadlineSmall,
            MATERIAL_TYPOGRAPHY.headline_small,
        ),
        (TypographyRole::TitleLarge, MATERIAL_TYPOGRAPHY.title_large),
        (
            TypographyRole::TitleMedium,
            MATERIAL_TYPOGRAPHY.title_medium,
        ),
        (TypographyRole::TitleSmall, MATERIAL_TYPOGRAPHY.title_small),
        (TypographyRole::LabelLarge, MATERIAL_TYPOGRAPHY.label_large),
        (
            TypographyRole::LabelMedium,
            MATERIAL_TYPOGRAPHY.label_medium,
        ),
        (
            TypographyRole::LabelMediumProminent,
            MATERIAL_TYPOGRAPHY.label_medium_prominent,
        ),
        (TypographyRole::LabelSmall, MATERIAL_TYPOGRAPHY.label_small),
        (TypographyRole::BodyLarge, MATERIAL_TYPOGRAPHY.body_large),
        (TypographyRole::BodyMedium, MATERIAL_TYPOGRAPHY.body_medium),
        (TypographyRole::BodySmall, MATERIAL_TYPOGRAPHY.body_small),
    ] {
        assert_eq!(text_style_for_role(role), expected);
    }
}
