use choreo_components_egui::choreography_settings::translations::ChoreographySettingsTranslations;

#[test]
fn choreography_settings_translations_bind_slint_catalog_values() {
    assert_eq!(ChoreographySettingsTranslations::comment("de"), "Kommentar");
    assert_eq!(
        ChoreographySettingsTranslations::choreography("de"),
        "Choreografie"
    );
    assert_eq!(
        ChoreographySettingsTranslations::selected_scene("de"),
        "Szene"
    );
}
