#![deny(warnings)]
#![deny(unsafe_code)]
#![deny(rust_2018_idioms)]
#![deny(unused_must_use)]
#![deny(unreachable_pub)]
#![deny(elided_lifetimes_in_paths)]
#![deny(clippy::all)]

mod clock;

pub mod errors;
pub mod models;
pub mod serialization;

pub use errors::ChoreoJsonError;
pub use models::*;
pub use serialization::{export, export_to_file, import, import_from_file};
