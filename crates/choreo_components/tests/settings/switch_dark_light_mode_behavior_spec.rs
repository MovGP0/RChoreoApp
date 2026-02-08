use std::time::Duration;

use crate::settings;

use choreo_components::preferences::Preferences;
use choreo_components::settings::ThemeMode;
use choreo_models::SettingsPreferenceKeys;
use settings::Report;

#[test]
#[serial_test::serial]
fn switch_dark_light_mode_behavior_spec() {
    let suite = rspec::describe("switch dark light mode behavior", (), |spec| {
        spec.it("updates use-system-theme flag and persists it", |_| {
            let context = settings::SettingsTestContext::new();

            context
                .view_model
                .borrow_mut()
                .update_use_system_theme(false);

            let updated = context.wait_until(Duration::from_secs(1), || {
                !context.view_model.borrow().use_system_theme
            });
            assert!(updated);
            assert!(
                !context
                    .preferences
                    .get_bool(SettingsPreferenceKeys::USE_SYSTEM_THEME, true)
            );
        });

        spec.it("switches theme mode and stores theme preference", |_| {
            let context = settings::SettingsTestContext::new();

            context.view_model.borrow_mut().update_is_dark_mode(true);

            let switched = context.wait_until(Duration::from_secs(1), || {
                context.view_model.borrow().theme_mode == ThemeMode::Dark
            });
            assert!(switched);
            assert_eq!(
                context
                    .preferences
                    .get_string(SettingsPreferenceKeys::THEME, ""),
                "Dark"
            );
            assert!(context.updater.call_count() >= 1);
        });
    });

    let report = settings::run_suite(&suite);
    assert!(report.is_success());
}
