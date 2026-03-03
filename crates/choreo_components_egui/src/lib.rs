#![deny(warnings)]
#![deny(unsafe_code)]
#![deny(rust_2018_idioms)]
#![deny(unused_must_use)]
#![deny(unreachable_pub)]
#![deny(elided_lifetimes_in_paths)]
#![deny(clippy::all)]

pub mod app_shell;
pub mod audio_player;
pub mod behavior;
pub mod choreo_main;
pub mod choreography_settings;
pub mod dancers;
pub mod dialog_host;
pub mod floor;
pub mod global;
pub mod haptics;
pub mod i18n;
pub mod logging;
pub mod main_page_drawer_host;
pub mod nav_bar;
pub mod observability;
pub mod preferences;
pub mod scenes;
pub mod settings;
pub mod shell;
pub mod slider_with_ticks;
pub mod splash_screen;
pub mod splash_screen_host;
pub mod time;
pub mod timestamp_state_machine;

pub use app_shell::AppShellViewModel;
