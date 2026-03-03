mod feedback;

pub trait HapticFeedback {
    fn is_supported(&self) -> bool;
    fn perform_click(&self);
}

pub type HapticFeedbackHandle = Option<Box<dyn HapticFeedback>>;

pub fn perform_click_if_supported(haptic: Option<&dyn HapticFeedback>) {
    if let Some(haptic) = haptic
        && haptic.is_supported()
    {
        haptic.perform_click();
    }
}

pub use feedback::NoopHapticFeedback;
pub use feedback::PlatformHapticFeedback;
