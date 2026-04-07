use crate::settings::Report;
use crate::settings::actions::SettingsAction;
use crate::settings::reducer::reduce;
use crate::settings::state::SettingsState;
use crate::settings::state::THEME_KEY;
use crate::settings::state::ThemeMode;
use crate::settings::state::USE_SYSTEM_THEME_KEY;

#[test]
fn switch_dark_light_mode_behavior_spec() {
    macro_rules! check_eq {
        ($errors:expr, $left:expr, $right:expr) => {
            if $left != $right {
                $errors.push(format!(
                    "{} != {} (left = {:?}, right = {:?})",
                    stringify!($left),
                    stringify!($right),
                    $left,
                    $right
                ));
            }
        };
    }

    macro_rules! check {
        ($errors:expr, $condition:expr) => {
            if !$condition {
                $errors.push(format!("condition failed: {}", stringify!($condition)));
            }
        };
    }

    let suite = rspec::describe("switch dark light mode reducer behavior", (), |spec| {
        spec.it("updates use-system-theme flag and persists it", |_| {
            let mut state = SettingsState::default();
            reduce(
                &mut state,
                SettingsAction::UpdateUseSystemTheme { enabled: false },
            );

            let mut errors = Vec::new();
            check!(errors, !state.use_system_theme);
            check_eq!(
                errors,
                state.preferences.get(USE_SYSTEM_THEME_KEY).map(String::as_str),
                Some("false")
            );

            assert!(
                errors.is_empty(),
                "Assertion failures:\n{}",
                errors.join("\n")
            );
        });

        spec.it("switches theme mode and stores preference", |_| {
            let mut state = SettingsState::default();
            reduce(
                &mut state,
                SettingsAction::UpdateIsDarkMode { enabled: true },
            );

            let mut errors = Vec::new();
            check_eq!(errors, state.theme_mode, ThemeMode::Dark);
            check_eq!(
                errors,
                state.preferences.get(THEME_KEY).map(String::as_str),
                Some("Dark")
            );

            assert!(
                errors.is_empty(),
                "Assertion failures:\n{}",
                errors.join("\n")
            );
        });
    });

    let report = crate::settings::run_suite(&suite);
    assert!(report.is_success());
}
