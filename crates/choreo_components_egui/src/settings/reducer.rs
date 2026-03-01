use super::actions::SettingsAction;
use super::state::SettingsState;
use super::state::AUDIO_PLAYER_BACKEND_KEY;
use super::state::AudioPlayerBackend;
use super::state::DEFAULT_PRIMARY_COLOR_HEX;
use super::state::DEFAULT_SECONDARY_COLOR_HEX;
use super::state::DEFAULT_TERTIARY_COLOR_HEX;
use super::state::PRIMARY_COLOR_KEY;
use super::state::SECONDARY_COLOR_KEY;
use super::state::TERTIARY_COLOR_KEY;
use super::state::THEME_KEY;
use super::state::ThemeMode;
use super::state::USE_PRIMARY_COLOR_KEY;
use super::state::USE_SECONDARY_COLOR_KEY;
use super::state::USE_SYSTEM_THEME_KEY;
use super::state::USE_TERTIARY_COLOR_KEY;

pub fn reduce(state: &mut SettingsState, action: SettingsAction) {
    match action {
        SettingsAction::Initialize => {}
        SettingsAction::LoadFromPreferences { entries } => {
            state.preferences = entries;
            apply_preferences(state);
        }
        SettingsAction::Reload => {
            apply_preferences(state);
        }
        SettingsAction::UpdateUseSystemTheme { enabled } => {
            state.use_system_theme = enabled;
            set_bool_pref(state, USE_SYSTEM_THEME_KEY, enabled);
            bump_material_update(state);
        }
        SettingsAction::UpdateIsDarkMode { enabled } => {
            state.theme_mode = if enabled {
                ThemeMode::Dark
            } else {
                ThemeMode::Light
            };
            state.preferences.insert(
                THEME_KEY.to_string(),
                if enabled {
                    "Dark".to_string()
                } else {
                    "Light".to_string()
                },
            );
            bump_material_update(state);
        }
        SettingsAction::UpdateUsePrimaryColor { enabled } => {
            state.use_primary_color = enabled;
            set_bool_pref(state, USE_PRIMARY_COLOR_KEY, enabled);
            if !enabled {
                state.use_secondary_color = false;
                state.use_tertiary_color = false;
                set_bool_pref(state, USE_SECONDARY_COLOR_KEY, false);
                set_bool_pref(state, USE_TERTIARY_COLOR_KEY, false);
                state.preferences.remove(PRIMARY_COLOR_KEY);
                state.preferences.remove(SECONDARY_COLOR_KEY);
                state.preferences.remove(TERTIARY_COLOR_KEY);
            }
            bump_material_update(state);
        }
        SettingsAction::UpdateUseSecondaryColor { enabled } => {
            if enabled && !state.use_primary_color {
                state.use_secondary_color = false;
                return;
            }
            state.use_secondary_color = enabled;
            set_bool_pref(state, USE_SECONDARY_COLOR_KEY, enabled);
            if !enabled {
                state.use_tertiary_color = false;
                set_bool_pref(state, USE_TERTIARY_COLOR_KEY, false);
                state.preferences.remove(SECONDARY_COLOR_KEY);
                state.preferences.remove(TERTIARY_COLOR_KEY);
            }
            bump_material_update(state);
        }
        SettingsAction::UpdateUseTertiaryColor { enabled } => {
            if enabled && !state.use_secondary_color {
                state.use_tertiary_color = false;
                return;
            }
            state.use_tertiary_color = enabled;
            set_bool_pref(state, USE_TERTIARY_COLOR_KEY, enabled);
            if !enabled {
                state.preferences.remove(TERTIARY_COLOR_KEY);
            }
            bump_material_update(state);
        }
        SettingsAction::UpdatePrimaryColorHex { value } => {
            if !is_argb_hex(value.trim()) {
                return;
            }
            state.primary_color_hex = value.trim().to_ascii_uppercase();
            if state.use_primary_color {
                state
                    .preferences
                    .insert(PRIMARY_COLOR_KEY.to_string(), state.primary_color_hex.clone());
                bump_material_update(state);
            }
        }
        SettingsAction::UpdateSecondaryColorHex { value } => {
            if !is_argb_hex(value.trim()) {
                return;
            }
            state.secondary_color_hex = value.trim().to_ascii_uppercase();
            if state.use_secondary_color {
                state.preferences.insert(
                    SECONDARY_COLOR_KEY.to_string(),
                    state.secondary_color_hex.clone(),
                );
                bump_material_update(state);
            }
        }
        SettingsAction::UpdateTertiaryColorHex { value } => {
            if !is_argb_hex(value.trim()) {
                return;
            }
            state.tertiary_color_hex = value.trim().to_ascii_uppercase();
            if state.use_tertiary_color {
                state.preferences.insert(
                    TERTIARY_COLOR_KEY.to_string(),
                    state.tertiary_color_hex.clone(),
                );
                bump_material_update(state);
            }
        }
        SettingsAction::UpdateAudioPlayerBackend { backend } => {
            state.audio_player_backend = backend;
            state.preferences.insert(
                AUDIO_PLAYER_BACKEND_KEY.to_string(),
                backend.as_preference().to_string(),
            );
        }
    }
}

fn apply_preferences(state: &mut SettingsState) {
    state.theme_mode = if state.preferences.get(THEME_KEY).is_some_and(|value| value == "Dark") {
        ThemeMode::Dark
    } else {
        ThemeMode::Light
    };
    state.use_system_theme = bool_pref(&state.preferences, USE_SYSTEM_THEME_KEY, true);
    state.use_primary_color = bool_pref(&state.preferences, USE_PRIMARY_COLOR_KEY, false);
    state.use_secondary_color = bool_pref(&state.preferences, USE_SECONDARY_COLOR_KEY, false)
        && state.use_primary_color;
    state.use_tertiary_color = bool_pref(&state.preferences, USE_TERTIARY_COLOR_KEY, false)
        && state.use_secondary_color;
    state.primary_color_hex = state
        .preferences
        .get(PRIMARY_COLOR_KEY)
        .filter(|value| is_argb_hex(value))
        .cloned()
        .unwrap_or_else(|| DEFAULT_PRIMARY_COLOR_HEX.to_string());
    state.secondary_color_hex = state
        .preferences
        .get(SECONDARY_COLOR_KEY)
        .filter(|value| is_argb_hex(value))
        .cloned()
        .unwrap_or_else(|| DEFAULT_SECONDARY_COLOR_HEX.to_string());
    state.tertiary_color_hex = state
        .preferences
        .get(TERTIARY_COLOR_KEY)
        .filter(|value| is_argb_hex(value))
        .cloned()
        .unwrap_or_else(|| DEFAULT_TERTIARY_COLOR_HEX.to_string());
    state.audio_player_backend = AudioPlayerBackend::from_preference(
        state
            .preferences
            .get(AUDIO_PLAYER_BACKEND_KEY)
            .map(String::as_str)
            .unwrap_or("rodio"),
    );
    bump_material_update(state);
}

fn bool_pref(
    prefs: &std::collections::BTreeMap<String, String>,
    key: &str,
    fallback: bool,
) -> bool {
    prefs
        .get(key)
        .map(String::as_str)
        .and_then(|value| value.parse::<bool>().ok())
        .unwrap_or(fallback)
}

fn set_bool_pref(state: &mut SettingsState, key: &str, value: bool) {
    state.preferences.insert(key.to_string(), value.to_string());
}

fn is_argb_hex(value: &str) -> bool {
    value.len() == 9
        && value.starts_with('#')
        && value
            .chars()
            .skip(1)
            .all(|character| character.is_ascii_hexdigit())
}

fn bump_material_update(state: &mut SettingsState) {
    state.material_update_count += 1;
}
