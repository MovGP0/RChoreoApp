use crate::settings::Report;
use crate::settings::actions::SettingsAction;
use crate::settings::reducer::reduce;
use crate::settings::state::SettingsState;
use crate::settings::state::THEME_KEY;
use crate::settings::state::ThemeMode;
use crate::settings::state::USE_SYSTEM_THEME_KEY;

#[test]
fn switch_dark_light_mode_behavior_spec() {
    let suite = rspec::describe("switch dark light mode reducer behavior", (), |spec| {
        spec.it("updates use-system-theme flag and persists it", |_| {
            let mut state = SettingsState::default();
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
        });

        spec.it("switches theme mode and stores preference", |_| {
            let mut state = SettingsState::default();
            reduce(&mut state, SettingsAction::UpdateIsDarkMode { enabled: true });

            assert_eq!(state.theme_mode, ThemeMode::Dark);
            assert_eq!(state.preferences.get(THEME_KEY).map(String::as_str), Some("Dark"));
        });
    });

    let report = crate::settings::run_suite(&suite);
    assert!(report.is_success());
}
