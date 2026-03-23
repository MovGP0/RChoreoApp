use crate::choreography_settings::create_state;
use choreo_components::choreography_settings::translations::ChoreographySettingsTranslations;
use choreo_components::choreography_settings::ui::uses_vertical_scroll_container;

const LOCALE: &str = "en";

#[test]
fn choreography_settings_sections_render_in_slint_card_order() {
    let state = create_state();
    let rendered = render_debug_shapes(&state);
    let expected_order = [
        ChoreographySettingsTranslations::selected_scene(LOCALE),
        ChoreographySettingsTranslations::display(LOCALE),
        ChoreographySettingsTranslations::choreography(LOCALE),
        ChoreographySettingsTranslations::floor(LOCALE),
    ];

    assert_labels_appear_in_order(&rendered, &expected_order);
}

#[test]
fn choreography_settings_panel_uses_vertical_scroll_container() {
    assert!(uses_vertical_scroll_container());
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

fn assert_labels_appear_in_order(rendered: &str, labels: &[String]) {
    let mut search_start = 0usize;
    for label in labels {
        let next_index = rendered[search_start..]
            .find(label)
            .map(|relative| search_start + relative)
            .unwrap_or_else(|| panic!("missing label in rendered output: {label}\n{rendered}"));
        search_start = next_index + label.len();
    }
}
