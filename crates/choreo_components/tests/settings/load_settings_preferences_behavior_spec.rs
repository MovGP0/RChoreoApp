use std::time::Duration;

use crate::settings;

use choreo_components::preferences::Preferences;
use choreo_components::settings::ThemeMode;
use choreo_models::SettingsPreferenceKeys;
use settings::Report;

#[test]
#[serial_test::serial]
fn load_settings_preferences_behavior_spec() {
    let suite = rspec::describe("load settings preferences behavior", (), |spec| {
        spec.it("loads persisted preferences on activation", |_| {
            let context = settings::SettingsTestContext::with_preferences(|preferences| {
                preferences.set_string(SettingsPreferenceKeys::THEME, "Dark".to_string());
                preferences.set_bool(SettingsPreferenceKeys::USE_SYSTEM_THEME, false);
                preferences.set_bool(SettingsPreferenceKeys::USE_PRIMARY_COLOR, true);
                preferences.set_bool(SettingsPreferenceKeys::USE_SECONDARY_COLOR, true);
                preferences.set_bool(SettingsPreferenceKeys::USE_TERTIARY_COLOR, true);
                preferences.set_string(
                    SettingsPreferenceKeys::PRIMARY_COLOR,
                    "#FF112233".to_string(),
                );
                preferences.set_string(
                    SettingsPreferenceKeys::SECONDARY_COLOR,
                    "#FF445566".to_string(),
                );
                preferences.set_string(
                    SettingsPreferenceKeys::TERTIARY_COLOR,
                    "#FF778899".to_string(),
                );
            });

            let loaded = context.wait_until(Duration::from_secs(1), || {
                let view_model = context.view_model.borrow();
                view_model.theme_mode == ThemeMode::Dark
                    && !view_model.use_system_theme
                    && view_model.use_primary_color
                    && view_model.use_secondary_color
                    && view_model.use_tertiary_color
            });
            assert!(loaded);
            assert!(context.updater.call_count() >= 1);
        });

        spec.it("reloads from updated preferences when reload is invoked", |_| {
            let context = settings::SettingsTestContext::new();
            context
                .preferences
                .set_string(SettingsPreferenceKeys::THEME, "Dark".to_string());
            context
                .preferences
                .set_bool(SettingsPreferenceKeys::USE_PRIMARY_COLOR, false);
            context
                .preferences
                .set_bool(SettingsPreferenceKeys::USE_SECONDARY_COLOR, true);
            context
                .preferences
                .set_string(SettingsPreferenceKeys::PRIMARY_COLOR, "invalid".to_string());

            context.view_model.borrow_mut().reload();

            let reloaded = context.wait_until(Duration::from_secs(1), || {
                let view_model = context.view_model.borrow();
                view_model.theme_mode == ThemeMode::Dark
            });
            assert!(reloaded);

            let view_model = context.view_model.borrow();
            assert!(!view_model.use_primary_color);
            assert!(!view_model.use_secondary_color);
            assert_eq!(
                view_model.primary_color.to_hex(),
                choreo_components::settings::default_primary_color().to_hex()
            );
        });
    });

    let report = settings::run_suite(&suite);
    assert!(report.is_success());
}
