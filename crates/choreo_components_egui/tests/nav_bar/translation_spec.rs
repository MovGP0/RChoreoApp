use choreo_components_egui::i18n::t;

#[test]
fn translation_helper_uses_fallback_when_key_is_missing() {
    let value = t("en", "MissingNavBarKey", "fallback-value");
    assert_eq!(value, "fallback-value");
}
