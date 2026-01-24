#![deny(warnings)]
#![deny(unsafe_code)]
#![deny(rust_2018_idioms)]
#![deny(unused_must_use)]
#![deny(unreachable_pub)]
#![deny(elided_lifetimes_in_paths)]
#![deny(clippy::all)]

slint::include_modules!();

pub fn hello_text() -> &'static str {
    "Hello from RChoreoApp"
}

pub fn create_main_window() -> Result<MainWindow, slint::PlatformError> {
    MainWindow::new()
}
