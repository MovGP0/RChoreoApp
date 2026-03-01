#![deny(warnings)]
#![deny(unsafe_code)]
#![deny(rust_2018_idioms)]
#![deny(unused_must_use)]
#![deny(unreachable_pub)]
#![deny(elided_lifetimes_in_paths)]
#![deny(clippy::all)]

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn main() {
    let _shell = choreo_components_egui::AppShellViewModel::new("ChoreoApp WASM egui");
    let _app_id = egui::Id::new("rchoreo_wasm_egui");

    #[cfg(target_arch = "wasm32")]
    web_sys::console::log_1(&"rchoreo_wasm_egui initialized".into());
}
