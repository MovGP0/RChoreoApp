use crate::choreography_settings::create_state;
use crate::choreography_settings::ui::choreo_date_text;
use crate::choreography_settings::ui::transparency_percentage_text;
use choreo_components_egui::choreo_info::state::ChoreoDate;
use choreo_components_egui::choreo_info::ui::picker_date_value;
use choreo_components_egui::choreo_info::ui::uses_calendar_date_picker;
use choreo_components_egui::material::components::DatePickerValue;

#[test]
fn choreo_info_date_text_is_zero_padded_iso_like() {
    let text = choreo_date_text(2026, 3, 1);
    assert_eq!(text, "2026-03-01");
}

#[test]
fn transparency_percentage_text_rounds_like_slint_math_round() {
    assert_eq!(transparency_percentage_text(0.0), "Transparency: 0%");
    assert_eq!(transparency_percentage_text(0.245), "Transparency: 25%");
    assert_eq!(transparency_percentage_text(0.999), "Transparency: 100%");
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

    let mut found_selected_scene_heading = false;
    for clipped in output.shapes {
        if format!("{:?}", clipped.shape).contains("Selected Scene") {
            found_selected_scene_heading = true;
            break;
        }
    }

    assert!(found_selected_scene_heading);
}

#[test]
fn selected_scene_controls_enabled_matches_scene_selection_state() {
    let mut state = create_state();
    state.has_selected_scene = false;
    assert!(!crate::choreography_settings::ui::selected_scene_controls_enabled(&state));

    state.has_selected_scene = true;
    assert!(crate::choreography_settings::ui::selected_scene_controls_enabled(&state));
}

#[test]
fn scene_timestamp_controls_enabled_requires_selected_scene_and_timestamp() {
    let mut state = create_state();

    state.has_selected_scene = false;
    state.scene_has_timestamp = false;
    assert!(!crate::choreography_settings::ui::scene_timestamp_controls_enabled(&state));

    state.has_selected_scene = true;
    state.scene_has_timestamp = false;
    assert!(!crate::choreography_settings::ui::scene_timestamp_controls_enabled(&state));

    state.has_selected_scene = false;
    state.scene_has_timestamp = true;
    assert!(!crate::choreography_settings::ui::scene_timestamp_controls_enabled(&state));

    state.has_selected_scene = true;
    state.scene_has_timestamp = true;
    assert!(crate::choreography_settings::ui::scene_timestamp_controls_enabled(&state));
}

#[test]
fn floor_size_maximum_uses_last_floor_option_like_slint_card() {
    let mut state = create_state();
    state.floor_size_options = vec![12, 24, 36, 48];

    assert_eq!(
        crate::choreography_settings::ui::floor_size_maximum(&state),
        48
    );

    state.floor_size_options.clear();
    assert_eq!(
        crate::choreography_settings::ui::floor_size_maximum(&state),
        100
    );
}
