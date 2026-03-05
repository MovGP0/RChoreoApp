#![deny(warnings)]
#![deny(unsafe_code)]
#![deny(rust_2018_idioms)]
#![deny(unused_must_use)]
#![deny(unreachable_pub)]
#![deny(elided_lifetimes_in_paths)]
#![deny(clippy::all)]

#[cfg(target_os = "android")]
use choreo_components_egui::AppShellViewModel;
#[cfg(target_os = "android")]
use choreo_components_egui::shell;

#[cfg(target_os = "android")]
const APP_TITLE: &str = "ChoreoApp Android egui";

#[cfg(target_os = "android")]
struct AndroidEguiApp {
    shell: AppShellViewModel,
}

#[cfg(target_os = "android")]
impl AndroidEguiApp {
    fn new(_creation_context: &eframe::CreationContext<'_>) -> Self {
        Self {
            shell: shell::create_shell_host(),
        }
    }
}

#[cfg(target_os = "android")]
impl eframe::App for AndroidEguiApp {
    fn update(&mut self, context: &egui::Context, _frame: &mut eframe::Frame) {
        self.shell.ui(context);
    }
}

#[cfg(target_os = "android")]
#[allow(unsafe_code)]
#[unsafe(no_mangle)]
pub fn android_main(app: winit::platform::android::activity::AndroidApp) {
    let native_options = eframe::NativeOptions {
        // Launcher icon packaging is provided through cargo-apk resources in
        // apps/android_egui/Cargo.toml and apps/android_egui/res.
        event_loop_builder: Some(Box::new(move |builder| {
            use winit::platform::android::EventLoopBuilderExtAndroid;

            builder.with_android_app(app);
        })),
        ..Default::default()
    };

    if let Err(error) = eframe::run_native(
        APP_TITLE,
        native_options,
        Box::new(|creation_context| Ok(Box::new(AndroidEguiApp::new(creation_context)))),
    ) {
        eprintln!("failed to run Android egui app: {error}");
    }
}

#[cfg(not(target_os = "android"))]
#[allow(dead_code)]
pub fn android_main(_app: ()) {
    // Non-Android builds keep a compile-safe entrypoint stub.
}
