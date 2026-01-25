#![deny(warnings)]
#![deny(unsafe_code)]
#![deny(rust_2018_idioms)]
#![deny(unused_must_use)]
#![deny(unreachable_pub)]
#![deny(elided_lifetimes_in_paths)]
#![deny(clippy::all)]

use slint::ComponentHandle;
use choreo_components::shell;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn main() {
    let ui = match shell::create_shell_host() {
        Ok(ui) => ui,
        Err(err) => {
            eprintln!("failed to create UI: {err}");
            return;
        }
    };
    ui.set_title_text(shell::app_title().into());
    if let Err(err) = ui.run() {
        eprintln!("failed to run UI: {err}");
    }
}
