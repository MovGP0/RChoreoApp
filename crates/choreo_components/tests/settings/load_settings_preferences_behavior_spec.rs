use std::collections::BTreeMap;

use crate::settings::Report;
use crate::settings::actions::SettingsAction;
use crate::settings::reducer::reduce;
use crate::settings::state::AUDIO_PLAYER_BACKEND_KEY;
use crate::settings::state::AudioPlayerBackend;
use crate::settings::state::PRIMARY_COLOR_KEY;
use crate::settings::state::SECONDARY_COLOR_KEY;
use crate::settings::state::SettingsState;
use crate::settings::state::TERTIARY_COLOR_KEY;
use crate::settings::state::THEME_KEY;
use crate::settings::state::ThemeMode;
use crate::settings::state::USE_PRIMARY_COLOR_KEY;
use crate::settings::state::USE_SECONDARY_COLOR_KEY;
use crate::settings::state::USE_SYSTEM_THEME_KEY;
use crate::settings::state::USE_TERTIARY_COLOR_KEY;

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

fn assert_no_errors(errors: Vec<String>) {
    assert!(
        errors.is_empty(),
        "expected no errors, but found:\n{}",
        errors.join("\n")
    );
}

#[test]
fn load_settings_preferences_behavior_spec() {
    let suite = rspec::describe("load settings preferences reducer behavior", (), |spec| {
        spec.it("loads persisted preferences on action", |_| {
            let mut state = SettingsState::default();
            let mut preferences = BTreeMap::new();
            preferences.insert(THEME_KEY.to_string(), "Dark".to_string());
            preferences.insert(USE_SYSTEM_THEME_KEY.to_string(), "false".to_string());
            preferences.insert(USE_PRIMARY_COLOR_KEY.to_string(), "true".to_string());
            preferences.insert(USE_SECONDARY_COLOR_KEY.to_string(), "true".to_string());
            preferences.insert(USE_TERTIARY_COLOR_KEY.to_string(), "true".to_string());
            preferences.insert(PRIMARY_COLOR_KEY.to_string(), "#FF112233".to_string());
            preferences.insert(SECONDARY_COLOR_KEY.to_string(), "#FF445566".to_string());
            preferences.insert(TERTIARY_COLOR_KEY.to_string(), "#FF778899".to_string());
            preferences.insert(AUDIO_PLAYER_BACKEND_KEY.to_string(), "awedio".to_string());

            reduce(
                &mut state,
                SettingsAction::LoadFromPreferences {
                    entries: preferences,
                },
            );

            let mut errors = Vec::new();
            check_eq!(errors, state.theme_mode, ThemeMode::Dark);
            check!(errors, !state.use_system_theme);
            check!(errors, state.use_primary_color);
            check!(errors, state.use_secondary_color);
            check!(errors, state.use_tertiary_color);
            check_eq!(errors, state.audio_player_backend, AudioPlayerBackend::Awedio);
            assert_no_errors(errors);
        });

        spec.it(
            "reload enforces hierarchy and falls back on invalid color",
            |_| {
                let mut state = SettingsState::default();
                let mut preferences = BTreeMap::new();
                preferences.insert(THEME_KEY.to_string(), "Dark".to_string());
                preferences.insert(USE_PRIMARY_COLOR_KEY.to_string(), "false".to_string());
                preferences.insert(USE_SECONDARY_COLOR_KEY.to_string(), "true".to_string());
                preferences.insert(PRIMARY_COLOR_KEY.to_string(), "invalid".to_string());
                reduce(
                    &mut state,
                    SettingsAction::LoadFromPreferences {
                        entries: preferences,
                    },
                );
                reduce(&mut state, SettingsAction::Reload);

                let mut errors = Vec::new();
                check_eq!(errors, state.theme_mode, ThemeMode::Dark);
                check!(errors, !state.use_primary_color);
                check!(errors, !state.use_secondary_color);
                check_eq!(errors, state.primary_color_hex, "#FF1976D2");
                assert_no_errors(errors);
            },
        );

        spec.it(
            "reload normalizes unsupported backends for the target",
            |_| {
                let mut state = SettingsState::default();
                let mut preferences = BTreeMap::new();
                preferences.insert(
                    AUDIO_PLAYER_BACKEND_KEY.to_string(),
                    if cfg!(target_arch = "wasm32") {
                        "awedio".to_string()
                    } else {
                        "browser".to_string()
                    },
                );

                reduce(
                    &mut state,
                    SettingsAction::LoadFromPreferences {
                        entries: preferences,
                    },
                );

                let mut errors = Vec::new();
                if cfg!(target_arch = "wasm32") {
                    check_eq!(errors, state.audio_player_backend, AudioPlayerBackend::Browser);
                } else {
                    check_eq!(errors, state.audio_player_backend, AudioPlayerBackend::Rodio);
                }
                assert_no_errors(errors);
            },
        );
    });

    let report = crate::settings::run_suite(&suite);
    assert!(report.is_success());
}
