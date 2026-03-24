use super::HapticFeedback;

#[derive(Clone, Copy, Default)]
pub struct NoopHapticFeedback;

impl NoopHapticFeedback {
    #[must_use]
    pub fn new() -> Self {
        Self
    }
}

impl HapticFeedback for NoopHapticFeedback {
    fn is_supported(&self) -> bool {
        false
    }

    fn perform_click(&self) {}
}

pub type PlatformHapticFeedback = NoopHapticFeedback;
