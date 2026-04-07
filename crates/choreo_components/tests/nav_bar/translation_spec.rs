use choreo_components::nav_bar::state::InteractionMode;
use choreo_components::nav_bar::translations::mode_text;
use choreo_components::nav_bar::translations::nav_bar_translations;

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
fn nav_bar_translations_bind_user_facing_strings() {
    let strings = nav_bar_translations("en");

    let mut errors = Vec::new();

    check_eq!(errors, strings.toggle_navigation_tooltip, "Toggle navigation");
    check_eq!(errors, strings.open_settings_tooltip, "Choreography Settings");
    check_eq!(errors, strings.reset_floor_viewport_tooltip, "Reset floor viewport");
    check_eq!(errors, strings.open_image_tooltip, "Open floor SVG");
    check_eq!(errors, strings.open_audio_tooltip, "Open audio file");
    check_eq!(errors, strings.mode_label, "Mode");

    assert_no_errors(errors);
}

#[test]
fn nav_bar_mode_translations_preserve_slint_mapping_intent() {
    let strings = nav_bar_translations("en");

    let mut errors = Vec::new();

    check_eq!(errors, mode_text(&strings, InteractionMode::View), "View");
    check_eq!(errors, mode_text(&strings, InteractionMode::Move), "Move");
    check_eq!(
        errors,
        mode_text(&strings, InteractionMode::RotateAroundCenter),
        "Rotate around center"
    );
    check_eq!(
        errors,
        mode_text(&strings, InteractionMode::RotateAroundDancer),
        "Rotate around dancer"
    );
    check_eq!(errors, mode_text(&strings, InteractionMode::Scale), "Scale");
    check_eq!(
        errors,
        mode_text(&strings, InteractionMode::LineOfSight),
        "Line of sight"
    );

    assert_no_errors(errors);
}
