use choreo_components_egui::i18n::slint_key_to_i18n_key;
use choreo_components_egui::i18n::t;
use choreo_components_egui::i18n::t_slint;
use choreo_components_egui::i18n::translate_slint;

#[test]
fn slint_keys_normalize_to_generated_i18n_catalog_keys() {
    assert_eq!(slint_key_to_i18n_key("app_title"), "AppTitle");
    assert_eq!(
        slint_key_to_i18n_key("dancer_swap_dialog_confirm"),
        "DancerSwapDialogConfirm"
    );
    assert_eq!(slint_key_to_i18n_key("common__cancel"), "CommonCancel");
}

#[test]
fn slint_translation_bridge_resolves_open_port_strings_from_catalog() {
    assert_eq!(t("en", "AppTitle"), "Choreography Viewer");
    assert_eq!(t_slint("en", "app_title"), "Choreography Viewer");
    assert_eq!(
        translate_slint("en", "app_title"),
        Some("Choreography Viewer")
    );
    assert_eq!(
        translate_slint("en", "dancer_swap_dialog_title"),
        Some("Swap dancers")
    );
    assert_eq!(
        translate_slint("en", "dancer_swap_dialog_confirm"),
        Some("Swap")
    );
    assert_eq!(translate_slint("en", "common_cancel"), Some("Cancel"));
}

#[test]
#[should_panic(expected = "Missing translation")]
fn slint_translation_bridge_panics_when_catalog_key_is_missing() {
    let _ = t_slint("en", "missing_shell_host_label");
}
