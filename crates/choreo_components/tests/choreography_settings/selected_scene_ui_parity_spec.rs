use crate::choreography_settings::create_state;
use choreo_components::choreography_settings::translations::ChoreographySettingsTranslations;

const LOCALE: &str = "en";

#[test]
fn selected_scene_card_renders_slint_parity_controls_when_scene_is_selected() {
    let mut state = create_state();
    state.has_selected_scene = true;
    state.scene_has_timestamp = true;

    let rendered = render_debug_shapes(&state);

    for expected_label in selected_scene_labels() {
        assert!(
            rendered.contains(&expected_label),
            "missing selected-scene control label: {expected_label}\nrendered: {rendered}"
        );
    }
}

fn render_debug_shapes(
    state: &choreo_components::choreography_settings::state::ChoreographySettingsState,
) -> String {
    let context = egui::Context::default();
    let output = context.run(egui::RawInput::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let _ = choreo_components::choreography_settings::ui::draw(ui, state);
        });
    });

    output
        .shapes
        .into_iter()
        .map(|clipped| format!("{:?}", clipped.shape))
        .collect::<Vec<_>>()
        .join("\n")
}

fn selected_scene_labels() -> Vec<String> {
    vec![
        ChoreographySettingsTranslations::selected_scene(LOCALE),
        ChoreographySettingsTranslations::scene_name(LOCALE),
        ChoreographySettingsTranslations::scene_text(LOCALE),
        ChoreographySettingsTranslations::scene_fixed_positions(LOCALE),
        ChoreographySettingsTranslations::scene_has_timestamp(LOCALE),
        ChoreographySettingsTranslations::timestamp_minutes(LOCALE),
        ChoreographySettingsTranslations::timestamp_seconds(LOCALE),
        ChoreographySettingsTranslations::timestamp_millis(LOCALE),
        ChoreographySettingsTranslations::scene_color(LOCALE),
    ]
}
