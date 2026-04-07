use choreo_components::i18n::slint_key_to_i18n_key;
use choreo_components::i18n::t;
use choreo_components::i18n::t_slint;
use choreo_components::i18n::translate_slint;

macro_rules! check_eq {
    ($errors:expr, $left:expr, $right:expr) => {
        if $left != $right {
            $errors.push(format!(
                "{} != {} (left = {:?}, right = {:?})",
                stringify!($left),
                stringify!($right),
                $left,
                $right
            ));
        }
    };
}

fn assert_no_errors(errors: Vec<String>) {
    assert!(
        errors.is_empty(),
        "Assertion failures:\n{}",
        errors.join("\n")
    );
}

#[test]
fn slint_keys_normalize_to_generated_i18n_catalog_keys() {
    let mut errors = Vec::new();

    check_eq!(errors, slint_key_to_i18n_key("app_title"), "AppTitle");
    check_eq!(
        errors,
        slint_key_to_i18n_key("dancer_swap_dialog_confirm"),
        "DancerSwapDialogConfirm"
    );
    assert_eq!(slint_key_to_i18n_key("common__cancel"), "CommonCancel");

    assert_no_errors(errors);
}

#[test]
fn slint_translation_bridge_resolves_open_port_strings_from_catalog() {
    let mut errors = Vec::new();

    check_eq!(errors, t("en", "AppTitle"), "Choreography Viewer");
    check_eq!(errors, t_slint("en", "app_title"), "Choreography Viewer");
    check_eq!(
        errors,
        translate_slint("en", "app_title"),
        Some("Choreography Viewer")
    );
    check_eq!(
        errors,
        translate_slint("en", "dancer_swap_dialog_title"),
        Some("Swap dancers")
    );
    check_eq!(
        errors,
        translate_slint("en", "dancer_swap_dialog_confirm"),
        Some("Swap")
    );
    assert_eq!(translate_slint("en", "common_cancel"), Some("Cancel"));

    assert_no_errors(errors);
}

#[test]
#[should_panic(expected = "Missing translation")]
fn slint_translation_bridge_panics_when_catalog_key_is_missing() {
    let _ = t_slint("en", "missing_shell_host_label");
}
