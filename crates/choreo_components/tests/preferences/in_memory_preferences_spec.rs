use crate::preferences::preferences::InMemoryPreferences;
use crate::preferences::preferences::Preferences;

#[test]
fn in_memory_preferences_store_strings_and_bools() {
    let prefs = InMemoryPreferences::new();
    prefs.set_string("name", "demo".to_string());
    prefs.set_bool("flag", true);

    assert_eq!(prefs.get_string("name", "fallback"), "demo");
    assert!(prefs.get_bool("flag", false));
}

#[test]
fn in_memory_preferences_remove_clears_values() {
    let prefs = InMemoryPreferences::new();
    prefs.set_string("k", "v".to_string());
    prefs.set_bool("k", true);
    prefs.remove("k");

    assert_eq!(prefs.get_string("k", "default"), "default");
    assert!(!prefs.get_bool("k", false));
}
