use choreo_components::choreography_settings::translations::ChoreographySettingsTranslations;
use choreo_components::choreography_settings::ui::GRID_RESOLUTION_DROPDOWN_ID;
use choreo_components::choreography_settings::ui::drawer_width_token;
use choreo_components::choreography_settings::ui::grid_resolution_dropdown_height_token;
use choreo_components::choreography_settings::ui::settings_card_content_width;
use choreo_components::choreography_settings::ui::settings_section_titles;
use choreo_components::choreography_settings::ui::uses_vertical_scroll_container;

const LOCALE: &str = "en";

#[test]
fn choreography_settings_sections_render_in_slint_card_order() {
    let expected_order = [
        ChoreographySettingsTranslations::selected_scene(LOCALE),
        ChoreographySettingsTranslations::display(LOCALE),
        ChoreographySettingsTranslations::choreography(LOCALE),
        ChoreographySettingsTranslations::floor(LOCALE),
    ];

    assert_eq!(settings_section_titles(LOCALE), expected_order);
}

#[test]
fn choreography_settings_panel_uses_vertical_scroll_container() {
    assert!(uses_vertical_scroll_container());
}

#[test]
fn choreography_settings_panel_exposes_a_fixed_drawer_width_token() {
    assert_eq!(drawer_width_token(), 360.0);
}

#[test]
fn choreography_settings_cards_use_the_drawer_width_as_their_outer_width() {
    assert_eq!(
        settings_card_content_width(drawer_width_token() - 8.0),
        326.0
    );
}

#[test]
fn grid_resolution_dropdown_menu_does_not_reserve_panel_layout_space() {
    let state = crate::choreography_settings::create_state();
    let context = egui::Context::default();
    let closed_height = render_choreography_settings_height(&context, &state);

    egui::Popup::open_id(
        &context,
        egui::Id::new(GRID_RESOLUTION_DROPDOWN_ID).with("popup"),
    );
    let open_height = render_choreography_settings_height(&context, &state);

    assert_eq!(grid_resolution_dropdown_height_token(), 56.0);
    assert_eq!(open_height, closed_height);
}

fn render_choreography_settings_height(
    context: &egui::Context,
    state: &choreo_components::choreography_settings::state::ChoreographySettingsState,
) -> u32 {
    let mut height = 0.0;
    let _ = context.run(egui::RawInput::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.set_width(drawer_width_token());
            let _ = choreo_components::choreography_settings::ui::draw(ui, state);
            height = ui.min_rect().height();
        });
    });

    height.round() as u32
}
