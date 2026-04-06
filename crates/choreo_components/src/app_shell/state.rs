use crate::splash_screen_host;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppShellState {
    pub title: String,
    pub is_main_page_initialized: bool,
    pub is_typography_initialized: bool,
    pub show_splash_screen: bool,
    pub splash_screen_state: splash_screen_host::ui::SplashScreenUiState,
}

impl Default for AppShellState {
    fn default() -> Self {
        Self {
            title: String::new(),
            is_main_page_initialized: false,
            is_typography_initialized: false,
            show_splash_screen: true,
            splash_screen_state: splash_screen_host::ui::SplashScreenUiState::default(),
        }
    }
}

impl AppShellState {
    #[must_use]
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            ..Self::default()
        }
    }
}
