mod feedback;

pub trait HapticFeedback {
    fn is_supported(&self) -> bool;
    fn perform_click(&self);
}

pub use feedback::{NoopHapticFeedback, PlatformHapticFeedback};
