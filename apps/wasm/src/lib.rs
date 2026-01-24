#![deny(warnings)]
#![deny(unsafe_code)]
#![deny(rust_2018_idioms)]
#![deny(unused_must_use)]
#![deny(unreachable_pub)]
#![deny(elided_lifetimes_in_paths)]
#![deny(clippy::all)]

slint::slint! {
    export component MainWindow inherits Window {
        in property<string> message;
        width: 360px;
        height: 240px;

        Text {
            text: message;
            color: #0f5132;
            font-size: 20px;
            horizontal-alignment: center;
            vertical-alignment: center;
        }
    }
}

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn main() {
    let ui = match MainWindow::new() {
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
