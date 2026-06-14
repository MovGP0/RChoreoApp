use super::state::SettingsState;
use super::state::ThemeMode;
use std::sync::Mutex;
use std::sync::OnceLock;
use std::time::Duration;
use std::time::Instant;

struct SystemThemeCache {
    checked_at: Instant,
    mode: Option<ThemeMode>,
}

static SYSTEM_THEME_CACHE: OnceLock<Mutex<Option<SystemThemeCache>>> = OnceLock::new();

#[must_use]
pub fn supports_system_theme_toggle() -> bool {
    !cfg!(target_arch = "wasm32")
}

#[must_use]
pub fn detect_system_theme_mode() -> Option<ThemeMode> {
    const REFRESH_INTERVAL: Duration = Duration::from_secs(1);
    let cache = SYSTEM_THEME_CACHE.get_or_init(|| Mutex::new(None));
    let mut state = cache
        .lock()
        .expect("system theme cache mutex should not be poisoned");

    if let Some(cached) = state.as_ref()
        && cached.checked_at.elapsed() < REFRESH_INTERVAL
    {
        return cached.mode;
    }

    let mode = detect_system_theme_mode_uncached();
    *state = Some(SystemThemeCache {
        checked_at: Instant::now(),
        mode,
    });
    mode
}

#[must_use]
pub fn effective_theme_mode(state: &SettingsState) -> ThemeMode {
    resolve_effective_theme_mode(state, detect_system_theme_mode())
}

#[must_use]
pub fn resolve_effective_theme_mode(
    state: &SettingsState,
    system_mode: Option<ThemeMode>,
) -> ThemeMode {
    if state.can_use_system_theme && state.use_system_theme {
        system_mode.unwrap_or(state.theme_mode)
    } else {
        state.theme_mode
    }
}

fn detect_system_theme_mode_uncached() -> Option<ThemeMode> {
    #[cfg(target_arch = "wasm32")]
    {
        None
    }

    #[cfg(target_os = "android")]
    {
        match dark_light::detect() {
            dark_light::Mode::Dark => Some(ThemeMode::Dark),
            dark_light::Mode::Light => Some(ThemeMode::Light),
            dark_light::Mode::Unspecified => None,
            #[allow(unreachable_patterns)]
            _ => None,
        }
    }

    #[cfg(all(not(target_arch = "wasm32"), not(target_os = "android")))]
    {
        match dark_light::detect().ok()? {
            dark_light::Mode::Dark => Some(ThemeMode::Dark),
            dark_light::Mode::Light => Some(ThemeMode::Light),
            dark_light::Mode::Unspecified => None,
            #[allow(unreachable_patterns)]
            _ => None,
        }
    }
}
