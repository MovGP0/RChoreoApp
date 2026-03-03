use std::path::PathBuf;

use serial_test::serial;

use crate::preferences::preferences::FilePreferences;
use crate::preferences::preferences::PlatformPreferences;
use crate::preferences::preferences::Preferences;

fn preferences_path(app_name: &str) -> PathBuf {
    let base_dir = dirs::config_dir()
        .or_else(|| std::env::current_dir().ok())
        .unwrap_or_else(|| PathBuf::from("."));
    base_dir.join(app_name).join("preferences.json")
}

fn test_app_name(test_case: &str) -> String {
    let pid = std::process::id();
    format!("rchoreo-egui-{test_case}-{pid}")
}

fn clear_preferences_file(app_name: &str) {
    let path = preferences_path(app_name);
    if let Some(parent) = path.parent() {
        let _ = std::fs::remove_dir_all(parent);
    }
}

#[test]
#[serial]
fn file_preferences_persist_across_instances() {
    let app_name = test_app_name("file");
    clear_preferences_file(&app_name);

    let first = FilePreferences::new(&app_name);
    first.set_string("theme", "material".to_string());
    first.set_bool("show_timestamps", true);

    let second = FilePreferences::new(&app_name);
    assert_eq!(second.get_string("theme", "fallback"), "material");
    assert!(second.get_bool("show_timestamps", false));

    clear_preferences_file(&app_name);
}

#[test]
#[serial]
fn platform_preferences_match_native_file_backend_contract() {
    let app_name = test_app_name("platform");
    clear_preferences_file(&app_name);

    let first = PlatformPreferences::new(&app_name);
    first.set_string("last_opened", "demo.choreo".to_string());
    first.set_bool("snap_to_grid", true);

    let second = PlatformPreferences::new(&app_name);
    assert_eq!(second.get_string("last_opened", ""), "demo.choreo");
    assert!(second.get_bool("snap_to_grid", false));

    second.remove("last_opened");
    second.remove("snap_to_grid");
    assert_eq!(second.get_string("last_opened", "default"), "default");
    assert!(!second.get_bool("snap_to_grid", false));

    clear_preferences_file(&app_name);
}
