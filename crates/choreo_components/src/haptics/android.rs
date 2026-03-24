#![cfg(target_os = "android")]

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
    #[must_use]
    #[allow(unsafe_code)]
    pub fn new() -> Self {
        let context = android_context();
        let vm = unsafe { JavaVM::from_raw(context.vm() as *mut jni::sys::JavaVM) };
        let global_context = vm
            .attach_current_thread(|env| {
                let context_object = unsafe { JObject::from_raw(env, context.context() as jobject) };
                env.new_global_ref(context_object)
            })
            .expect("Failed to create global Android context ref");

        Self {
            vm,
            context: global_context,
        }
    }

    fn with_context<R>(
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
        self.with_context(|env, context| {
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

pub type PlatformHapticFeedback = AndroidHapticFeedback;
