use std::time::Duration;

use crate::settings;

use choreo_components::preferences::Preferences;
use choreo_models::SettingsPreferenceKeys;
use settings::Report;

#[test]
#[serial_test::serial]
fn color_preferences_behavior_spec() {
    let suite = rspec::describe("color preferences behavior", (), |spec| {
        spec.it("enforces primary-secondary-tertiary enablement hierarchy", |_| {
                let context = settings::SettingsTestContext::new();

                context
                    .view_model
                    .borrow_mut()
                    .update_use_secondary_color(true);
                context.pump_events();
                assert!(!context.view_model.borrow().use_secondary_color);

                context
                    .view_model
                    .borrow_mut()
                    .update_use_primary_color(true);
                let primary_enabled = context.wait_until(Duration::from_secs(1), || {
                    context.view_model.borrow().use_primary_color
                });
                assert!(primary_enabled);

                context
                    .view_model
                    .borrow_mut()
                    .update_use_secondary_color(true);
                context
                    .view_model
                    .borrow_mut()
                    .update_use_tertiary_color(true);
                let all_enabled = context.wait_until(Duration::from_secs(1), || {
                    let view_model = context.view_model.borrow();
                    view_model.use_secondary_color && view_model.use_tertiary_color
                });
                assert!(all_enabled);

                context
                    .view_model
                    .borrow_mut()
                    .update_use_primary_color(false);
                let reset = context.wait_until(Duration::from_secs(1), || {
                    let view_model = context.view_model.borrow();
                    !view_model.use_primary_color
                        && !view_model.use_secondary_color
                        && !view_model.use_tertiary_color
                });
                assert!(reset);
                assert_eq!(
                    context
                        .preferences
                        .get_string(SettingsPreferenceKeys::PRIMARY_COLOR, ""),
                    ""
                );
                assert_eq!(
                    context
                        .preferences
                        .get_string(SettingsPreferenceKeys::SECONDARY_COLOR, ""),
                    ""
                );
                assert_eq!(
                    context
                        .preferences
                        .get_string(SettingsPreferenceKeys::TERTIARY_COLOR, ""),
                    ""
                );
            },
        );

        spec.it("stores color hex only for enabled color channels", |_| {
            let context = settings::SettingsTestContext::new();

            context
                .view_model
                .borrow_mut()
                .update_primary_color_hex("#FF336699".to_string());
            context.pump_events();
            assert_eq!(
                context
                    .preferences
                    .get_string(SettingsPreferenceKeys::PRIMARY_COLOR, ""),
                ""
            );

            context
                .view_model
                .borrow_mut()
                .update_use_primary_color(true);
            let enabled = context.wait_until(Duration::from_secs(1), || {
                context.view_model.borrow().use_primary_color
            });
            assert!(enabled);

            context
                .view_model
                .borrow_mut()
                .update_primary_color_hex("#FF336699".to_string());
            let stored = context.wait_until(Duration::from_secs(1), || {
                context
                    .preferences
                    .get_string(SettingsPreferenceKeys::PRIMARY_COLOR, "")
                    == "#FF336699"
            });
            assert!(stored);
            assert_eq!(
                context.view_model.borrow().primary_color.to_hex(),
                "#FF336699"
            );
        });
    });

    let report = settings::run_suite(&suite);
    assert!(report.is_success());
}
