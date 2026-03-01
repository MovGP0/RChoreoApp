#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HapticBackend {
    Noop,
    Platform,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HapticEffect {
    Click,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HapticsState {
    pub backend: HapticBackend,
    pub supported: bool,
    pub trigger_count: u64,
    pub delivered_count: u64,
    pub pending_effect: Option<HapticEffect>,
}

impl Default for HapticsState {
    fn default() -> Self {
        Self {
            backend: HapticBackend::Noop,
            supported: false,
            trigger_count: 0,
            delivered_count: 0,
            pending_effect: None,
        }
    }
}
