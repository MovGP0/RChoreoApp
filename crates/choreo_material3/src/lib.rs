#![deny(warnings)]
#![deny(unsafe_code)]
#![deny(rust_2018_idioms)]
#![deny(unused_must_use)]
#![deny(unreachable_pub)]
#![deny(elided_lifetimes_in_paths)]
#![deny(clippy::all)]

pub mod components;
pub mod icons;
pub mod items;
pub mod styling;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ThemeMode {
    #[default]
    Light,
    Dark,
}

/// Registers image loaders so embedded SVG assets can render in egui views.
pub fn install_image_loaders(context: &egui::Context) {
    egui_extras::install_image_loaders(context);
}
