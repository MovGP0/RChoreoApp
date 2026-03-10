use crate::settings::Report;
use crate::settings::actions::SettingsAction;
use crate::settings::reducer::reduce;
use crate::settings::state::PRIMARY_COLOR_KEY;
use crate::settings::state::SettingsState;
use crate::settings::state::THEME_KEY;
use crate::settings::state::ThemeMode;
use crate::settings::state::USE_SYSTEM_THEME_KEY;

#[test]
fn material_theme_application_spec() {
    let suite = rspec::describe("material theme reducer integration", (), |spec| {
        spec.it("applies dark and light theme from toggle inputs", |_| {
            let mut state = SettingsState::default();
            let baseline = state.material_update_count;
            reduce(
                &mut state,
                SettingsAction::UpdateUseSystemTheme { enabled: false },
            );
            assert!(!state.use_system_theme);
            assert_eq!(
                state
                    .preferences
                    .get(USE_SYSTEM_THEME_KEY)
                    .map(String::as_str),
                Some("false")
            );

            reduce(
                &mut state,
                SettingsAction::UpdateIsDarkMode { enabled: true },
            );
            assert_eq!(state.theme_mode, ThemeMode::Dark);
            assert_eq!(
                state.preferences.get(THEME_KEY).map(String::as_str),
                Some("Dark")
            );
            assert_eq!(
                choreo_components_egui::material::styling::material_palette::material_palette_for_settings_state(&state)
                    .background,
                state.material_scheme.dark.background
            );

            reduce(
                &mut state,
                SettingsAction::UpdateIsDarkMode { enabled: false },
            );
            assert_eq!(state.theme_mode, ThemeMode::Light);
            assert_eq!(
                state.preferences.get(THEME_KEY).map(String::as_str),
                Some("Light")
            );
            assert_eq!(
                choreo_components_egui::material::styling::material_palette::material_palette_for_settings_state(&state)
                    .background,
                state.material_scheme.light.background
            );

            assert_eq!(state.material_update_count, baseline + 3);
        });

        spec.it(
            "recalculates dynamic material roles when custom colors change",
            |_| {
                let mut state = SettingsState::default();
                let baseline_primary = state.material_scheme.light.primary;
                let baseline_secondary = state.material_scheme.dark.secondary;

                reduce(
                    &mut state,
                    SettingsAction::UpdateUsePrimaryColor { enabled: true },
                );
                reduce(
                    &mut state,
                    SettingsAction::UpdatePrimaryColorHex {
                        value: "#FF336699".to_string(),
                    },
                );

                assert_eq!(
                    state.preferences.get(PRIMARY_COLOR_KEY).map(String::as_str),
                    Some("#FF336699")
                );
                assert_ne!(state.material_scheme.light.primary, baseline_primary);

                reduce(
                    &mut state,
                    SettingsAction::UpdateUseSecondaryColor { enabled: true },
                );
                reduce(
                    &mut state,
                    SettingsAction::UpdateSecondaryColorHex {
                        value: "#FF884422".to_string(),
                    },
                );

                assert_ne!(state.material_scheme.dark.secondary, baseline_secondary);
            },
        );
    });

    let report = crate::settings::run_suite(&suite);
    assert!(report.is_success());
}
