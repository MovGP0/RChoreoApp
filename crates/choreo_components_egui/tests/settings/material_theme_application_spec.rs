use crate::settings::Report;
use crate::settings::actions::SettingsAction;
use crate::settings::reducer::reduce;
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

            reduce(
                &mut state,
                SettingsAction::UpdateIsDarkMode { enabled: false },
            );
            assert_eq!(state.theme_mode, ThemeMode::Light);
            assert_eq!(
                state.preferences.get(THEME_KEY).map(String::as_str),
                Some("Light")
            );

            assert_eq!(state.material_update_count, baseline + 3);
        });
    });

    let report = crate::settings::run_suite(&suite);
    assert!(report.is_success());
}
