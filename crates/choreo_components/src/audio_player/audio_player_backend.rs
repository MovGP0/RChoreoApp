#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioPlayerBackend {
    Rodio,
    Awedio,
    Browser,
}

impl AudioPlayerBackend {
    pub const RODIO_KEY: &'static str = "Rodio";
    pub const AWEDIO_KEY: &'static str = "Awedio";
    pub const BROWSER_KEY: &'static str = "Browser";

    #[must_use]
    pub fn available_for_current_target() -> &'static [Self] {
        #[cfg(target_arch = "wasm32")]
        {
            &[Self::Browser]
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            &[Self::Rodio, Self::Awedio]
        }
    }

    #[must_use]
    pub fn default_for_current_target() -> Self {
        Self::available_for_current_target()[0]
    }

    #[must_use]
    pub fn normalize_for_current_target(self) -> Self {
        if Self::available_for_current_target().contains(&self) {
            self
        } else {
            Self::default_for_current_target()
        }
    }

    #[must_use]
    pub fn from_preference(value: &str) -> Self {
        let normalized = value.trim().to_ascii_lowercase();
        if normalized == Self::BROWSER_KEY.to_ascii_lowercase() {
            return Self::Browser.normalize_for_current_target();
        }
        if normalized == Self::AWEDIO_KEY.to_ascii_lowercase() {
            return Self::Awedio.normalize_for_current_target();
        }
        Self::Rodio.normalize_for_current_target()
    }

    #[must_use]
    pub fn as_preference(self) -> &'static str {
        match self {
            Self::Rodio => Self::RODIO_KEY,
            Self::Awedio => Self::AWEDIO_KEY,
            Self::Browser => Self::BROWSER_KEY,
        }
    }
}
