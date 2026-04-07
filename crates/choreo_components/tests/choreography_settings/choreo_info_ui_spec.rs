use crate::choreography_settings::create_state;
use crate::choreography_settings::ui::choreo_date_text;
use crate::choreography_settings::ui::transparency_percentage_text;
use choreo_components::choreo_info::state::ChoreoDate;
use choreo_components::choreo_info::ui::picker_date_value;
use choreo_components::choreo_info::ui::uses_calendar_date_picker;
use choreo_components::choreography_settings::translations::ChoreographySettingsTranslations;
use choreo_components::material::components::DatePickerValue;

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

macro_rules! check_ne {
    ($errors:expr, $left:expr, $right:expr) => {
        if $left == $right {
            $errors.push(format!(
                "{} == {} (left = {:?}, right = {:?})",
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
fn choreo_info_date_text_is_zero_padded_iso_like() {
    let text = choreo_date_text(2026, 3, 1);
    let mut errors = Vec::new();

    check_eq!(errors, text, "2026-03-01");

    assert_no_errors(errors);
}

#[test]
fn transparency_percentage_text_rounds_like_slint_math_round() {
    let mut errors = Vec::new();

    check_eq!(
        errors,
        transparency_percentage_text(0.0),
        "Transparency: 0%"
    );
    check_eq!(
        errors,
        transparency_percentage_text(0.245),
        "Transparency: 25%"
    );
    check_eq!(
        errors,
        transparency_percentage_text(0.999),
        "Transparency: 100%"
    );

    assert_no_errors(errors);
}

#[test]
fn choreography_settings_uses_calendar_date_picker() {
    assert!(uses_calendar_date_picker());
}

#[test]
fn picker_date_value_falls_back_for_invalid_parts() {
    let value = picker_date_value(ChoreoDate {
        year: 2026,
        month: 2,
        day: 31,
    });

    assert_ne!(
        value,
        DatePickerValue {
            year: 2026,
            month: 2,
            day: 31,
        }
    );
}

#[test]
fn choreography_settings_ui_draw_handles_choreo_info_inputs() {
    let mut state = create_state();
    state.comment = "Comment".to_string();
    state.name = "Name".to_string();
    state.subtitle = "Subtitle".to_string();
    state.date.year = 2025;
    state.date.month = 12;
    state.date.day = 31;
    state.variation = "Variation".to_string();
    state.author = "Author".to_string();
    state.description = "Description".to_string();
    state.transparency = 0.42;

    let context = egui::Context::default();
    let _ = context.run(egui::RawInput::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let _ = crate::choreography_settings::ui::draw(ui, &state);
        });
    });
}

#[test]
fn selected_scene_section_renders_even_without_selected_scene() {
    let mut state = create_state();
    state.has_selected_scene = false;

    let context = egui::Context::default();
    let output = context.run(egui::RawInput::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let _ = crate::choreography_settings::ui::draw(ui, &state);
        });
    });

    let expected_heading = ChoreographySettingsTranslations::selected_scene("en");
    let mut found_selected_scene_heading = false;
    for clipped in output.shapes {
        if format!("{:?}", clipped.shape).contains(expected_heading.as_str()) {
            found_selected_scene_heading = true;
            break;
        }
    }

    assert!(found_selected_scene_heading);
}

#[test]
fn selected_scene_controls_enabled_matches_scene_selection_state() {
    let mut state = create_state();
    let mut errors = Vec::new();

    state.has_selected_scene = false;
    check!(
        errors,
        !crate::choreography_settings::ui::selected_scene_controls_enabled(&state)
    );

    state.has_selected_scene = true;
    check!(
        errors,
        crate::choreography_settings::ui::selected_scene_controls_enabled(&state)
    );

    assert_no_errors(errors);
}

#[test]
fn scene_timestamp_controls_enabled_requires_selected_scene_and_timestamp() {
    let mut state = create_state();
    let mut errors = Vec::new();

    state.has_selected_scene = false;
    state.scene_has_timestamp = false;
    check!(
        errors,
        !crate::choreography_settings::ui::scene_timestamp_controls_enabled(&state)
    );

    state.has_selected_scene = true;
    state.scene_has_timestamp = false;
    check!(
        errors,
        !crate::choreography_settings::ui::scene_timestamp_controls_enabled(&state)
    );

    state.has_selected_scene = false;
    state.scene_has_timestamp = true;
    check!(
        errors,
        !crate::choreography_settings::ui::scene_timestamp_controls_enabled(&state)
    );

    state.has_selected_scene = true;
    state.scene_has_timestamp = true;
    check!(
        errors,
        crate::choreography_settings::ui::scene_timestamp_controls_enabled(&state)
    );

    assert_no_errors(errors);
}

#[test]
fn floor_size_maximum_uses_last_floor_option_like_slint_card() {
    let mut state = create_state();
    let mut errors = Vec::new();

    state.floor_size_options = vec![12, 24, 36, 48];

    check_eq!(
        errors,
        crate::choreography_settings::ui::floor_size_maximum(&state),
        48
    );

    state.floor_size_options.clear();
    check_eq!(
        errors,
        crate::choreography_settings::ui::floor_size_maximum(&state),
        100
    );

    assert_no_errors(errors);
}
