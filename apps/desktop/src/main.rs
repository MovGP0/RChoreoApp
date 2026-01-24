#![deny(warnings)]
#![deny(unsafe_code)]
#![deny(rust_2018_idioms)]
#![deny(unused_must_use)]
#![deny(unreachable_pub)]
#![deny(elided_lifetimes_in_paths)]
#![deny(clippy::all)]

use slint::ComponentHandle;

fn main() -> Result<(), slint::PlatformError> {
    let ui = shared::create_main_window()?;
    ui.set_message(shared::hello_text().into());
    ui.run()
}
