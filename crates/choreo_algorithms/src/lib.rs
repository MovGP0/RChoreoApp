#![deny(warnings)]
#![deny(unsafe_code)]
#![deny(rust_2018_idioms)]
#![deny(unused_must_use)]
#![deny(unreachable_pub)]
#![deny(elided_lifetimes_in_paths)]
#![deny(clippy::all)]

pub mod error;
pub mod hungarian;
pub mod min_cost_max_flow;
mod vector2;

pub use error::AlgorithmError;
pub use vector2::Vector2;
