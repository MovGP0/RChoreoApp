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

            assert_eq!(state.theme_mode, ThemeMode::Dark);
            assert!(!state.use_system_theme);
            assert!(state.use_primary_color);
            assert!(state.use_secondary_color);
            assert!(state.use_tertiary_color);
            assert_eq!(state.audio_player_backend, AudioPlayerBackend::Awedio);
        });

        spec.it("reload enforces hierarchy and falls back on invalid color", |_| {
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

            assert_eq!(state.theme_mode, ThemeMode::Dark);
            assert!(!state.use_primary_color);
            assert!(!state.use_secondary_color);
            assert_eq!(state.primary_color_hex, "#FF1976D2");
        });
    });

    let report = crate::settings::run_suite(&suite);
    assert!(report.is_success());
}
