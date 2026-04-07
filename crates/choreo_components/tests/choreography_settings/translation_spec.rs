use choreo_components::choreography_settings::translations::ChoreographySettingsTranslations;

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
fn choreography_settings_translations_bind_slint_catalog_values() {
    let mut errors = Vec::new();

    check_eq!(
        errors,
        ChoreographySettingsTranslations::comment("de"),
        "Kommentar"
    );
    check_eq!(
        errors,
        ChoreographySettingsTranslations::choreography("de"),
        "Choreografie"
    );
    check_eq!(
        errors,
        ChoreographySettingsTranslations::selected_scene("de"),
        "Szene"
    );

    assert_no_errors(errors);
}
