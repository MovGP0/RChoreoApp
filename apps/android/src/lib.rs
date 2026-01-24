#![deny(warnings)]
#![deny(unsafe_code)]
#![deny(rust_2018_idioms)]
#![deny(unused_must_use)]
#![deny(unreachable_pub)]
#![deny(elided_lifetimes_in_paths)]
#![deny(clippy::all)]

#[allow(unsafe_code)]
#[unsafe(no_mangle)]
#[cfg(target_os = "android")]
fn android_main(app: slint::android::AndroidApp) {
    if let Err(err) = slint::android::init(app) {
        eprintln!("failed to init Slint Android backend: {err}");
        return;
    }
    let ui = match shared::create_main_window() {
        Ok(ui) => ui,
        Err(err) => {
            eprintln!("failed to create UI: {err}");
            return;
        }
    };
    ui.set_message(shared::hello_text().into());
    if let Err(err) = ui.run() {
        eprintln!("failed to run UI: {err}");
    }
}

#[cfg(not(target_os = "android"))]
#[allow(dead_code)]
fn android_main(_: ()) {
    // Non-Android builds should not attempt to use the Android backend.
}
