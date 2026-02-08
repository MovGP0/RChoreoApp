use crate::choreography_settings;

use choreo_components::behavior::Behavior;
use choreo_components::choreography_settings::LoadSettingsPreferencesBehavior;
use choreo_components::preferences::InMemoryPreferences;
use choreo_components::preferences::Preferences;
use choreo_models::SettingsModel;
use choreo_models::SettingsPreferenceKeys;
use choreography_settings::Report;

#[test]
#[serial_test::serial]
fn load_settings_preferences_behavior_spec() {
    let suite = rspec::describe("load settings preferences behavior", (), |spec| {
        spec.it("loads booleans from preferences into settings model", |_| {
            let preferences = InMemoryPreferences::new();
            preferences.set_bool(SettingsPreferenceKeys::SHOW_TIMESTAMPS, false);
            preferences.set_bool(SettingsPreferenceKeys::POSITIONS_AT_SIDE, false);
            preferences.set_bool(SettingsPreferenceKeys::SNAP_TO_GRID, true);

            let behavior = LoadSettingsPreferencesBehavior::new(preferences);
            let mut settings = SettingsModel::default();
            let mut disposables = choreo_components::behavior::CompositeDisposable::new();
            behavior.activate(&mut settings, &mut disposables);

            assert!(!settings.show_timestamps);
            assert!(!settings.positions_at_side);
            assert!(settings.snap_to_grid);
        });
    });

    let report = choreography_settings::run_suite(&suite);
    assert!(report.is_success());
}
