#![deny(warnings)]
#![deny(unsafe_code)]
#![deny(rust_2018_idioms)]
#![deny(unused_must_use)]
#![deny(unreachable_pub)]
#![deny(elided_lifetimes_in_paths)]
#![deny(clippy::all)]

use slint::ComponentHandle;
use choreo_components::shell;

fn main() -> Result<(), slint::PlatformError> {
    let ui = shell::create_shell_host()?;
    ui.set_title_text(shell::app_title().into());
    ui.run()
}
