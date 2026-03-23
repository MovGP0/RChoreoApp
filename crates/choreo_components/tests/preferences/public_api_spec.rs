#[test]
fn preferences_public_api_is_exported_from_crate_root() {
    let prefs = choreo_components::preferences::InMemoryPreferences::new();
    let _shared = choreo_components::preferences::SharedPreferences::new(std::rc::Rc::new(
        prefs.clone(),
    ));
    let _platform: choreo_components::preferences::PlatformPreferences =
        choreo_components::preferences::PlatformPreferences::new("rchoreo");
}
