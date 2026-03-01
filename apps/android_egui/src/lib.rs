#![deny(warnings)]
#![deny(unsafe_code)]
#![deny(rust_2018_idioms)]
#![deny(unused_must_use)]
#![deny(unreachable_pub)]
#![deny(elided_lifetimes_in_paths)]
#![deny(clippy::all)]

#[cfg(target_os = "android")]
#[allow(unsafe_code)]
#[unsafe(no_mangle)]
pub fn android_main(_app: ()) {
    let _shell = choreo_components_egui::AppShellViewModel::new("ChoreoApp Android egui");
}

#[cfg(not(target_os = "android"))]
#[allow(dead_code)]
pub fn android_main(_app: ()) {
    let _shell = choreo_components_egui::AppShellViewModel::new("ChoreoApp Android egui");
}
