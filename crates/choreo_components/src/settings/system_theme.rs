use super::ThemeMode;
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant};

struct SystemThemeCache {
    checked_at: Instant,
    mode: Option<ThemeMode>,
}

static SYSTEM_THEME_CACHE: OnceLock<Mutex<Option<SystemThemeCache>>> = OnceLock::new();

pub fn supports_system_theme_toggle() -> bool {
    !cfg!(target_arch = "wasm32")
}

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

fn detect_system_theme_mode_uncached() -> Option<ThemeMode> {
    match dark_light::detect().ok()? {
        dark_light::Mode::Dark => Some(ThemeMode::Dark),
        dark_light::Mode::Light => Some(ThemeMode::Light),
        dark_light::Mode::Unspecified => None,
        #[allow(unreachable_patterns)]
        _ => None,
    }
}
