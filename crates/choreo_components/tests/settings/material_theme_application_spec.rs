use crate::settings::Report;
use crate::settings::actions::SettingsAction;
use crate::settings::reducer::reduce;
use crate::settings::state::PRIMARY_COLOR_KEY;
use crate::settings::state::SettingsState;
use crate::settings::state::THEME_KEY;
use crate::settings::state::ThemeMode;
use crate::settings::state::USE_SYSTEM_THEME_KEY;

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

macro_rules! check_ne {
    ($errors:expr, $left:expr, $right:expr) => {
        if $left == $right {
            $errors.push(format!(
                "{} == {} (left = {:?}, right = {:?})",
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

fn assert_no_errors(errors: Vec<String>) {
    assert!(
        errors.is_empty(),
        "Assertion failures:\n{}",
        errors.join("\n")
    );
}

#[test]
fn material_theme_application_spec() {
    let suite = rspec::describe("material theme reducer integration", (), |spec| {
        spec.it("applies dark and light theme from toggle inputs", |_| {
            let mut state = SettingsState::default();
            let baseline = state.material_update_count;
            let mut errors = Vec::new();

            reduce(
                &mut state,
                SettingsAction::UpdateUseSystemTheme { enabled: false },
            );
            check!(&mut errors, !state.use_system_theme);
            check_eq!(
                errors,
                state.preferences.get(USE_SYSTEM_THEME_KEY).map(String::as_str),
                Some("false")
            );

            reduce(
                &mut state,
                SettingsAction::UpdateIsDarkMode { enabled: true },
            );
            check_eq!(errors, state.theme_mode, ThemeMode::Dark);
            check_eq!(
                errors,
                state.preferences.get(THEME_KEY).map(String::as_str),
                Some("Dark")
            );
            check_eq!(
                errors,
                choreo_components::material::styling::material_palette::material_palette_for_theme(
                    &state.material_scheme,
                    state.theme_mode,
                )
                .background,
                state.material_scheme.dark.background
            );

            reduce(
                &mut state,
                SettingsAction::UpdateIsDarkMode { enabled: false },
            );
            check_eq!(errors, state.theme_mode, ThemeMode::Light);
            check_eq!(
                errors,
                state.preferences.get(THEME_KEY).map(String::as_str),
                Some("Light")
            );
            check_eq!(
                errors,
                choreo_components::material::styling::material_palette::material_palette_for_theme(
                    &state.material_scheme,
                    state.theme_mode,
                )
                .background,
                state.material_scheme.light.background
            );

            check_eq!(errors, state.material_update_count, baseline + 3);

            assert_no_errors(errors);
        });

        spec.it(
            "recalculates dynamic material roles when custom colors change",
            |_| {
                let mut state = SettingsState::default();
                let baseline_primary = state.material_scheme.light.primary;
                let baseline_secondary = state.material_scheme.dark.secondary;
                let mut errors = Vec::new();

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

                check_eq!(
                    errors,
                    state.preferences.get(PRIMARY_COLOR_KEY).map(String::as_str),
                    Some("#FF336699")
                );
                check_ne!(errors, state.material_scheme.light.primary, baseline_primary);

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

                check_ne!(
                    errors,
                    state.material_scheme.dark.secondary,
                    baseline_secondary
                );

                assert_no_errors(errors);
            },
        );
    });

    let report = crate::settings::run_suite(&suite);
    assert!(report.is_success());
}
