use crate::settings::state::AudioPlayerBackend;
use crate::settings::state::SettingsState;
use crate::settings::translations::settings_translations;
use crate::settings::ui::audio_backend_label;
use crate::settings::ui::available_audio_backends_for_current_target;
use crate::settings::ui::card_corner_radius_token;
use crate::settings::ui::card_padding_token;
use crate::settings::ui::card_spacing_token;
use crate::settings::ui::color_picker_state_from_argb_hex;
use crate::settings::ui::color_picker_wheel_size_token;
use crate::settings::ui::color_swatch_height_token;
use crate::settings::ui::color_swatch_width_token;
use crate::settings::ui::content_max_width_token;
use crate::settings::ui::content_spacing_token;
use crate::settings::ui::dropdown_height_token;
use crate::settings::ui::format_argb_hex;
use crate::settings::ui::navigate_back_icon_name;
use crate::settings::ui::navigate_back_icon_svg;
use crate::settings::ui::parse_argb_hex;
use crate::settings::ui::selected_theme_mode_dropdown_index;
use crate::settings::ui::shows_audio_backend_card;
use crate::settings::ui::theme_mode_dropdown_labels;
use crate::settings::ui::visible_settings_card_headers;
use choreo_components::material::styling::material_style_metrics::material_style_metrics;

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

macro_rules! check {
    ($errors:expr, $condition:expr) => {
        if !$condition {
            $errors.push(format!("condition failed: {}", stringify!($condition)));
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
fn settings_ui_draw_executes_without_panicking() {
    let state = SettingsState::default();
    let context = egui::Context::default();
    let _ = context.run(egui::RawInput::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let _ = crate::settings::ui::draw(ui, &state);
        });
    });
}

#[test]
fn audio_backend_labels_match_expected_values() {
    let strings = settings_translations("en");
    let mut errors = Vec::new();

    check_eq!(
        errors,
        audio_backend_label(AudioPlayerBackend::Rodio, &strings),
        "Rodio"
    );
    check_eq!(
        errors,
        audio_backend_label(AudioPlayerBackend::Awedio, &strings),
        "Awedio"
    );
    check_eq!(
        errors,
        audio_backend_label(AudioPlayerBackend::Browser, &strings),
        "Browser"
    );

    assert_no_errors(errors);
}

#[test]
fn audio_backend_visibility_matches_current_target() {
    let backends = available_audio_backends_for_current_target();
    let mut errors = Vec::new();

    if cfg!(target_arch = "wasm32") {
        check_eq!(errors, backends, vec![AudioPlayerBackend::Browser]);
        check!(errors, !shows_audio_backend_card());
    } else {
        check_eq!(
            errors,
            backends,
            vec![AudioPlayerBackend::Rodio, AudioPlayerBackend::Awedio]
        );
        check!(errors, shows_audio_backend_card());
    }

    assert_no_errors(errors);
}

#[test]
fn settings_translations_bind_slint_catalog_values() {
    let strings = settings_translations("de");
    let mut errors = Vec::new();

    check_eq!(errors, strings.title, "Einstellungen");
    check_eq!(
        errors,
        strings.use_system_theme,
        "Systemdesign verwenden"
    );

    assert_no_errors(errors);
}

#[test]
fn theme_mode_dropdown_labels_follow_requested_selection_order() {
    let strings = settings_translations("en");
    let mut state = SettingsState::default();
    let mut errors = Vec::new();

    check_eq!(
        errors,
        theme_mode_dropdown_labels(&state, &strings),
        vec![
            "Use system theme".to_string(),
            "Dark mode".to_string(),
            "Light mode".to_string(),
        ]
    );
    check_eq!(errors, selected_theme_mode_dropdown_index(&state), 0);

    state.use_system_theme = false;
    state.theme_mode = crate::settings::state::ThemeMode::Dark;
    check_eq!(errors, selected_theme_mode_dropdown_index(&state), 1);

    state.can_use_system_theme = false;
    state.theme_mode = crate::settings::state::ThemeMode::Light;
    check_eq!(
        errors,
        theme_mode_dropdown_labels(&state, &strings),
        vec!["Dark mode".to_string(), "Light mode".to_string()]
    );
    check_eq!(errors, selected_theme_mode_dropdown_index(&state), 1);

    assert_no_errors(errors);
}

#[test]
fn settings_card_headers_follow_requested_order() {
    let state = SettingsState::default();
    let strings = settings_translations("en");
    let mut expected = vec![
        "Theme".to_string(),
        "Primary color".to_string(),
        "Secondary color".to_string(),
        "Tertiary color".to_string(),
    ];

    if shows_audio_backend_card() {
        expected.push("Audio backend".to_string());
    }

    assert_eq!(visible_settings_card_headers(&state, &strings), expected);
}

#[test]
fn parse_argb_hex_handles_valid_and_invalid_values() {
    assert!(parse_argb_hex("#FF112233").is_some());
    assert!(parse_argb_hex("#112233").is_none());
    assert!(parse_argb_hex("#GG112233").is_none());
}

#[test]
fn format_argb_hex_round_trips_with_parse_helper() {
    let color = egui::Color32::from_rgb(0x11, 0x22, 0x33);

    let formatted = format_argb_hex(color);

    assert_eq!(formatted, "#FF112233");
    assert_eq!(parse_argb_hex(&formatted), Some(color));
}

#[test]
fn color_picker_state_uses_parsed_argb_value() {
    let picker_state = color_picker_state_from_argb_hex("#FF336699");
    let mut errors = Vec::new();

    check_eq!(
        errors,
        picker_state.selected_color,
        egui::Color32::from_rgba_unmultiplied(0x33, 0x66, 0x99, 0xFF)
    );
    check_eq!(
        errors,
        picker_state.value_slider_position,
        material3::components::color_picker::state::ColorPickerDock::Bottom
    );
    check_eq!(
        errors,
        picker_state.wheel_minimum_width,
        color_picker_wheel_size_token()
    );
    check_eq!(
        errors,
        picker_state.wheel_minimum_height,
        color_picker_wheel_size_token()
    );

    assert_no_errors(errors);
}

#[test]
fn navigate_back_control_uses_icon_catalog_mapping() {
    let mut errors = Vec::new();

    check_eq!(errors, navigate_back_icon_name(), "arrow_back");
    check!(errors, navigate_back_icon_svg().contains("<svg"));

    assert_no_errors(errors);
}

#[test]
fn settings_spacing_uses_shared_material_metrics_token() {
    assert_eq!(
        content_spacing_token(),
        material_style_metrics().spacings.spacing_12
    );
}

#[test]
fn settings_layout_tokens_match_slint_card_structure() {
    let metrics = material_style_metrics();
    let mut errors = Vec::new();

    check_eq!(errors, card_spacing_token(), metrics.spacings.spacing_12);
    check_eq!(errors, card_padding_token(), metrics.paddings.padding_12);
    check_eq!(
        errors,
        card_corner_radius_token(),
        metrics.corner_radii.border_radius_8
    );
    check_eq!(errors, content_max_width_token(), 720.0);
    check_eq!(errors, dropdown_height_token(), metrics.sizes.size_56);
    check_eq!(errors, color_swatch_width_token(), 108.0);
    check_eq!(errors, color_swatch_height_token(), 36.0);
    check_eq!(errors, color_picker_wheel_size_token(), 168.0);

    assert_no_errors(errors);
}
