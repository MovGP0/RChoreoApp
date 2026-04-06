use choreo_components::choreography_settings::translations::ChoreographySettingsTranslations;
use choreo_components::choreography_settings::ui::drawer_width_token;
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
