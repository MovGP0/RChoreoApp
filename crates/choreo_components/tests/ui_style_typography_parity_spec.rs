use choreo_components::audio_player::ui as audio_ui;
use choreo_components::main_page::ui as main_page_ui;
use choreo_components::material::styling::material_typography::TypographyRole;
use choreo_components::material::styling::material_typography::apply_text_styles;
use choreo_components::material::styling::material_typography::font_id_for_role;
use choreo_components::material::styling::material_typography::platform_font_fallback_chain;
use choreo_components::material::styling::material_typography::style_for_role;
use choreo_components::nav_bar::ui as nav_ui;
use choreo_components::scenes::ui as scenes_ui;
use choreo_components::settings::ui as settings_ui;
use egui::FontFamily;
use egui::FontId;
use egui::Style;
use egui::TextStyle;

macro_rules! check_eq {
    ($errors:expr, $left:expr, $right:expr) => {
        if $left != $right {
            $errors.push(format!(
                "{} != {} (left = {:?}, right = {:?})",
                stringify!($left),
                stringify!($right),
                $left,
                $right
            ));
        }
    };
}

fn assert_no_errors(errors: Vec<String>) {
    assert!(
        errors.is_empty(),
        "Assertion failures:\n{}",
        errors.join("\n")
    );
}

#[test]
fn material_typography_roles_match_slint_scale() {
    let mut errors = Vec::new();

    assert_style(&mut errors, TypographyRole::DisplayLarge, 57.0, 64.0, 300);
    assert_style(&mut errors, TypographyRole::DisplayMedium, 45.0, 52.0, 300);
    assert_style(&mut errors, TypographyRole::DisplaySmall, 36.0, 44.0, 300);
    assert_style(&mut errors, TypographyRole::HeadlineLarge, 32.0, 40.0, 300);
    assert_style(&mut errors, TypographyRole::HeadlineMedium, 28.0, 36.0, 300);
    assert_style(&mut errors, TypographyRole::HeadlineSmall, 24.0, 32.0, 300);
    assert_style(&mut errors, TypographyRole::TitleLarge, 22.0, 28.0, 300);
    assert_style(&mut errors, TypographyRole::TitleMedium, 16.0, 24.0, 600);
    assert_style(&mut errors, TypographyRole::TitleSmall, 14.0, 20.0, 600);
    assert_style(&mut errors, TypographyRole::LabelLarge, 14.0, 20.0, 600);
    assert_style(&mut errors, TypographyRole::LabelMedium, 12.0, 16.0, 600);
    assert_style(
        &mut errors,
        TypographyRole::LabelMediumProminent,
        12.0,
        16.0,
        900
    );
    assert_style(&mut errors, TypographyRole::LabelSmall, 11.0, 16.0, 600);
    assert_style(&mut errors, TypographyRole::BodyLarge, 16.0, 24.0, 300);
    assert_style(&mut errors, TypographyRole::BodyMedium, 14.0, 20.0, 300);
    assert_style(&mut errors, TypographyRole::BodySmall, 12.0, 16.0, 300);

    assert_no_errors(errors);
}

#[test]
fn typography_declares_platform_safe_fallback_chain() {
    assert_eq!(
        platform_font_fallback_chain(),
        &[
            "Roboto",
            "Segoe UI",
            "Noto Sans",
            "Helvetica Neue",
            "Arial",
            "sans-serif"
        ]
    );
}

#[test]
fn typography_roles_are_assigned_for_primary_surfaces() {
    let mut errors = Vec::new();

    check_eq!(errors, nav_ui::mode_label_role(), TypographyRole::LabelLarge);
    check_eq!(errors, main_page_ui::mode_label_role(), TypographyRole::LabelLarge);
    check_eq!(
        errors,
        settings_ui::page_title_role(),
        TypographyRole::HeadlineSmall
    );
    check_eq!(
        errors,
        settings_ui::section_label_role(),
        TypographyRole::TitleSmall
    );
    check_eq!(errors, audio_ui::speed_label_role(), TypographyRole::BodyMedium);
    check_eq!(
        errors,
        audio_ui::duration_label_role(),
        TypographyRole::BodyMedium
    );
    check_eq!(errors, scenes_ui::scene_title_role(), TypographyRole::BodyMedium);
    check_eq!(
        errors,
        scenes_ui::scene_timestamp_role(),
        TypographyRole::LabelMedium
    );

    assert_no_errors(errors);
}

#[test]
fn typography_applies_shared_egui_text_style_defaults() {
    let style = apply_text_styles(&Style::default());
    let mut errors = Vec::new();

    check_eq!(
        errors,
        style.text_styles.get(&TextStyle::Heading),
        Some(&font_id_for_role(TypographyRole::HeadlineSmall))
    );
    check_eq!(
        errors,
        style.text_styles.get(&TextStyle::Body),
        Some(&font_id_for_role(TypographyRole::BodyMedium))
    );
    check_eq!(
        errors,
        style.text_styles.get(&TextStyle::Button),
        Some(&font_id_for_role(TypographyRole::LabelLarge))
    );
    check_eq!(
        errors,
        style.text_styles.get(&TextStyle::Small),
        Some(&font_id_for_role(TypographyRole::BodySmall))
    );
    check_eq!(
        errors,
        style.text_styles.get(&TextStyle::Monospace),
        Some(&FontId::new(12.0, FontFamily::Monospace))
    );

    assert_no_errors(errors);
}

fn assert_style(
    errors: &mut Vec<String>,
    role: TypographyRole,
    expected_size_px: f32,
    expected_line_height_px: f32,
    expected_weight: i32,
) {
    let style = style_for_role(role);

    check_eq!(errors, style.font_size_px, expected_size_px);
    check_eq!(errors, style.line_height_px, expected_line_height_px);
    check_eq!(errors, style.font_weight, expected_weight);
}
