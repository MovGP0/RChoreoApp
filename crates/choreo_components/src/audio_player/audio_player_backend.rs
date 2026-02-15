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

    pub fn from_preference(value: &str) -> Self {
        if cfg!(target_arch = "wasm32") {
            return Self::Browser;
        }

        let normalized = value.trim().to_ascii_lowercase();
        if normalized == Self::AWEDIO_KEY.to_ascii_lowercase() {
            return Self::Awedio;
        }
        Self::Rodio
    }

    pub fn as_preference(self) -> &'static str {
        match self {
            Self::Rodio => Self::RODIO_KEY,
            Self::Awedio => Self::AWEDIO_KEY,
            Self::Browser => Self::BROWSER_KEY,
        }
    }
}
