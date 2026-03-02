#![deny(warnings)]
#![deny(unsafe_code)]
#![deny(rust_2018_idioms)]
#![deny(unused_must_use)]
#![deny(unreachable_pub)]
#![deny(elided_lifetimes_in_paths)]
#![deny(clippy::all)]

pub mod app_shell;
pub mod audio_player;
pub mod choreo_main;
pub mod choreography_settings;
pub mod dialog_host;
pub mod floor;
pub mod main_page_drawer_host;
pub mod observability;
pub mod scenes;
pub mod settings;
pub mod shell;
pub mod time;

pub use app_shell::AppShellViewModel;
