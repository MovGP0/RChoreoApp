use choreo_components::choreo_info::messages::ChoreoInfoAction;
use choreo_components::choreo_info::state::ChoreoInfoState;
use choreo_components::choreo_info::ui::ChoreoInfoLabels;
use choreo_components::choreo_info::ui::choreo_date_text;
use choreo_components::choreo_info::ui::draw;
use choreo_components::choreo_info::ui::draw_transparency;
use choreo_components::choreo_info::ui::transparency_percentage_text;

#[test]
fn choreo_info_state_defaults_match_slint_global_defaults() {
    let state = ChoreoInfoState::default();
    assert_eq!(state.choreo_name, "Lorem ipsum dolor sit amet");
    assert_eq!(state.choreo_subtitle, "consectetur adipiscing elit");
    assert_eq!(state.choreo_comment, "");
    assert_eq!(state.choreo_variation, "");
    assert_eq!(state.choreo_author, "");
    assert_eq!(state.choreo_description, "");
    assert_eq!(state.choreo_transparency, 0.0);
}

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
fn transparency_draw_renders_shared_label_and_percentage() {
    let context = egui::Context::default();
    let output = context.run(egui::RawInput::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let action = draw_transparency(ui, 0.42, "Transparency");
            assert_eq!(action, None);
        });
    });

    let mut found_transparency = false;
    for clipped in output.shapes {
        if format!("{:?}", clipped.shape).contains("Transparency: 42%") {
            found_transparency = true;
            break;
        }
    }

    assert!(found_transparency);
}

#[test]
fn transparency_action_variant_preserves_fractional_value() {
    let action = ChoreoInfoAction::UpdateTransparency(0.42);
    assert_eq!(action, ChoreoInfoAction::UpdateTransparency(0.42));
}

#[test]
fn choreo_info_ui_draw_handles_all_information_inputs() {
    let state = ChoreoInfoState {
        choreo_name: "Name".to_string(),
        choreo_subtitle: "Subtitle".to_string(),
        choreo_comment: "Comment".to_string(),
        choreo_date: choreo_components::choreo_info::state::ChoreoDate {
            year: 2025,
            month: 12,
            day: 31,
        },
        choreo_variation: "Variation".to_string(),
        choreo_author: "Author".to_string(),
        choreo_description: "Description".to_string(),
        choreo_transparency: 0.42,
    };

    let labels = ChoreoInfoLabels {
        comment: "Comment".to_string(),
        name: "Name".to_string(),
        subtitle: "Subtitle".to_string(),
        date: "Date".to_string(),
        variation: "Variation".to_string(),
        author: "Author".to_string(),
        description: "Description".to_string(),
    };

    let context = egui::Context::default();
    let _ = context.run(egui::RawInput::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let _ = draw(ui, &state, &labels);
        });
    });
}
