#![deny(warnings)]
#![deny(unsafe_code)]
#![deny(rust_2018_idioms)]
#![deny(unused_must_use)]
#![deny(unreachable_pub)]
#![deny(elided_lifetimes_in_paths)]
#![deny(clippy::all)]

pub mod audio_player;
pub mod behavior;
pub mod choreo_main;
pub mod choreography_settings;
pub mod color_picker;
pub mod dancers;
pub mod date;
pub mod floor;
pub mod global;
pub mod haptics;
pub mod i18n;
pub mod logging;
pub mod nav_bar;
pub mod preferences;
pub mod scenes;
pub mod settings;
pub mod shell;
pub mod slider_with_ticks;
pub mod splash_screen;
pub mod time;

mod ui {
    #![allow(unreachable_pub)]
    slint::include_modules!();
}

pub use ui::*;
