#[cfg(target_os = "android")]
mod android;
#[cfg(target_os = "ios")]
mod ios;
mod noop;

pub trait HapticFeedback {
    fn is_supported(&self) -> bool;
    fn perform_click(&self);
}

pub use noop::NoopHapticFeedback;

#[cfg(target_os = "android")]
pub use android::PlatformHapticFeedback;
#[cfg(target_os = "ios")]
pub use ios::PlatformHapticFeedback;
#[cfg(all(not(target_os = "android"), not(target_os = "ios")))]
pub use noop::PlatformHapticFeedback;
