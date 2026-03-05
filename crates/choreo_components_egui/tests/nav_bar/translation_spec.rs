use choreo_components_egui::nav_bar::state::InteractionMode;
use choreo_components_egui::nav_bar::translations::mode_text;
use choreo_components_egui::nav_bar::translations::nav_bar_translations;

#[test]
fn nav_bar_translations_bind_user_facing_strings() {
    let strings = nav_bar_translations("en");

    assert_eq!(strings.toggle_navigation_tooltip, "Toggle navigation");
    assert_eq!(strings.open_settings_tooltip, "Choreography Settings");
    assert_eq!(strings.reset_floor_viewport_tooltip, "Reset floor viewport");
    assert_eq!(strings.open_image_tooltip, "Open floor SVG");
    assert_eq!(strings.open_audio_tooltip, "Open audio file");
    assert_eq!(strings.mode_label, "Mode");
}

#[test]
fn nav_bar_mode_translations_preserve_slint_mapping_intent() {
    let strings = nav_bar_translations("en");

    assert_eq!(mode_text(&strings, InteractionMode::View), "View");
    assert_eq!(mode_text(&strings, InteractionMode::Move), "Move");
    assert_eq!(
        mode_text(&strings, InteractionMode::RotateAroundCenter),
        "Rotate around center"
    );
    assert_eq!(
        mode_text(&strings, InteractionMode::RotateAroundDancer),
        "Rotate around dancer"
    );
    assert_eq!(mode_text(&strings, InteractionMode::Scale), "Scale");
    assert_eq!(
        mode_text(&strings, InteractionMode::LineOfSight),
        "Line of sight"
    );
}
