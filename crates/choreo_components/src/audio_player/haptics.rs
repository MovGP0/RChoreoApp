use super::HapticFeedback;

#[derive(Clone, Copy, Default)]
pub struct NoopHapticFeedback;

impl NoopHapticFeedback {
    pub fn new() -> Self
    {
        Self
    }
}

impl HapticFeedback for NoopHapticFeedback {
    fn is_supported(&self) -> bool
    {
        false
    }

    fn perform_click(&self)
    {
    }
}

#[cfg(target_os = "android")]
mod android {
    use jni::objects::{GlobalRef, JObject, JValue};
    use jni::sys::jobject;
    use jni::JavaVM;
    use ndk_context::android_context;

    use super::HapticFeedback;

    pub struct AndroidHapticFeedback {
        vm: JavaVM,
        context: GlobalRef,
    }

    impl AndroidHapticFeedback {
        pub fn new() -> Self
        {
            let context = android_context();
            let vm = unsafe { JavaVM::from_raw(context.vm()) }
                .expect("Failed to attach to Android JVM");
            let env = vm
                .attach_current_thread()
                .expect("Failed to attach Android thread");
            let context_obj = unsafe { JObject::from_raw(context.context() as jobject) };
            let global_context = env
                .new_global_ref(context_obj)
                .expect("Failed to create global Android context ref");
            std::mem::forget(context_obj);

            Self {
                vm,
                context: global_context,
            }
        }

        fn with_env<R>(&self, action: impl FnOnce(&jni::JNIEnv, JObject) -> Option<R>) -> Option<R>
        {
            let env = self.vm.attach_current_thread().ok()?;
            action(&env, self.context.as_obj())
        }

        fn with_vibrator<R>(
            &self,
            action: impl FnOnce(&jni::JNIEnv, JObject) -> Option<R>,
        ) -> Option<R>
        {
            self.with_env(|env, context| {
                let service_name = env.new_string("vibrator").ok()?;
                let vibrator = env
                    .call_method(
                        context,
                        "getSystemService",
                        "(Ljava/lang/String;)Ljava/lang/Object;",
                        &[JValue::Object(&service_name)],
                    )
                    .ok()?
                    .l()
                    .ok()?;
                action(env, vibrator)
            })
        }
    }

    impl HapticFeedback for AndroidHapticFeedback {
        fn is_supported(&self) -> bool
        {
            self.with_vibrator(|env, vibrator| {
                env.call_method(vibrator, "hasVibrator", "()Z", &[])
                    .ok()?
                    .z()
                    .ok()
            })
            .unwrap_or(false)
        }

        fn perform_click(&self)
        {
            let _ = self.with_vibrator(|env, vibrator| {
                let _ = env.call_method(
                    vibrator,
                    "vibrate",
                    "(J)V",
                    &[JValue::Long(10)],
                );
                Some(())
            });
        }
    }
}

#[cfg(target_os = "android")]
pub use android::AndroidHapticFeedback;

#[cfg(target_os = "ios")]
mod ios {
    use objc::runtime::Object;
    use objc::{class, msg_send, sel, sel_impl};

    use super::HapticFeedback;

    pub struct IosHapticFeedback;

    impl IosHapticFeedback {
        pub fn new() -> Self
        {
            Self
        }
    }

    impl HapticFeedback for IosHapticFeedback {
        fn is_supported(&self) -> bool
        {
            true
        }

        fn perform_click(&self)
        {
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
