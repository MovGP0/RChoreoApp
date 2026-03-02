use crate::choreography_settings::create_state;
use crate::choreography_settings::ui::choreo_date_text;
use crate::choreography_settings::ui::transparency_percentage_text;

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
