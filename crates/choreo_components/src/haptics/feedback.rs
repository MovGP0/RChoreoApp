use super::HapticFeedback;

#[derive(Clone, Copy, Default)]
pub struct NoopHapticFeedback;

impl NoopHapticFeedback {
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

#[cfg(target_os = "android")]
mod android {
    use jni::JavaVM;
    use jni::objects::{Global, JObject, JValue};
    use jni::sys::jobject;
    use ndk_context::android_context;

    use super::HapticFeedback;

    pub struct AndroidHapticFeedback {
        vm: JavaVM,
        context: Global<JObject<'static>>,
    }

    impl AndroidHapticFeedback {
        pub fn new() -> Self {
            let context = android_context();
            let vm = unsafe { JavaVM::from_raw(context.vm() as *mut jni::sys::JavaVM) };
            let global_context = vm
                .attach_current_thread(|env| {
                    let context_object =
                        unsafe { JObject::from_raw(env, context.context() as jobject) };
                    env.new_global_ref(context_object)
                })
                .expect("Failed to create global Android context ref");

            Self {
                vm,
                context: global_context,
            }
        }

        fn with_env<R>(
            &self,
            action: impl FnOnce(&mut jni::Env<'_>, &JObject<'static>) -> jni::errors::Result<R>,
        ) -> Option<R> {
            self.vm
                .attach_current_thread(|env| action(env, self.context.as_obj()))
                .ok()
        }

        fn with_vibrator<R>(
            &self,
            action: impl FnOnce(&mut jni::Env<'_>, JObject<'_>) -> jni::errors::Result<R>,
        ) -> Option<R> {
            self.with_env(|env, context| {
                let service_name = env.new_string("vibrator")?;
                let vibrator = env
                    .call_method(
                        context,
                        jni::jni_str!("getSystemService"),
                        jni::jni_sig!("(Ljava/lang/String;)Ljava/lang/Object;"),
                        &[JValue::Object(&service_name)],
                    )?
                    .l()?;
                action(env, vibrator)
            })
        }
    }

    impl Default for AndroidHapticFeedback {
        fn default() -> Self {
            Self::new()
        }
    }

    impl HapticFeedback for AndroidHapticFeedback {
        fn is_supported(&self) -> bool {
            self.with_vibrator(|env, vibrator| {
                env.call_method(
                    vibrator,
                    jni::jni_str!("hasVibrator"),
                    jni::jni_sig!("()Z"),
                    &[],
                )?
                .z()
            })
            .unwrap_or(false)
        }

        fn perform_click(&self) {
            let _ = self.with_vibrator(|env, vibrator| {
                let _ = env.call_method(
                    vibrator,
                    jni::jni_str!("vibrate"),
                    jni::jni_sig!("(J)V"),
                    &[JValue::Long(10)],
                );
                Ok(())
            });
        }
    }
}

#[cfg(target_os = "android")]
use android::AndroidHapticFeedback;

#[cfg(target_os = "ios")]
mod ios {
    use objc::runtime::Object;
    use objc::{class, msg_send, sel, sel_impl};

    use super::HapticFeedback;

    pub struct IosHapticFeedback;

    impl IosHapticFeedback {
        pub fn new() -> Self {
            Self
        }
    }

    impl HapticFeedback for IosHapticFeedback {
        fn is_supported(&self) -> bool {
            true
        }

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
}

#[cfg(target_os = "ios")]
pub use ios::IosHapticFeedback;

#[cfg(target_os = "android")]
pub type PlatformHapticFeedback = AndroidHapticFeedback;
#[cfg(target_os = "ios")]
pub type PlatformHapticFeedback = IosHapticFeedback;
#[cfg(all(not(target_os = "android"), not(target_os = "ios")))]
pub type PlatformHapticFeedback = NoopHapticFeedback;
