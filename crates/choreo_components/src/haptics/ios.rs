#![cfg(target_os = "ios")]

use objc::class;
use objc::msg_send;
use objc::runtime::Object;

use super::HapticFeedback;

pub struct IosHapticFeedback;

impl IosHapticFeedback {
    #[must_use]
    pub fn new() -> Self {
        Self
    }
}

impl HapticFeedback for IosHapticFeedback {
    fn is_supported(&self) -> bool {
        true
    }

    #[allow(unsafe_code)]
    fn perform_click(&self) {
        unsafe {
            let generator: *mut Object = msg_send![class!(UIImpactFeedbackGenerator), alloc];
            let generator: *mut Object = msg_send![generator, initWithStyle: 0];
            let _: () = msg_send![generator, prepare];
            let _: () = msg_send![generator, impactOccurred];
            let _: () = msg_send![generator, release];
        }
    }
}

pub type PlatformHapticFeedback = IosHapticFeedback;
