#![deny(warnings)]
#![deny(unsafe_code)]
#![deny(rust_2018_idioms)]
#![deny(unused_must_use)]
#![deny(unreachable_pub)]
#![deny(elided_lifetimes_in_paths)]
#![deny(clippy::all)]

mod clone_mode;
mod colors;
mod mapping;
mod models;
mod preferences;

pub use clone_mode::CloneMode;
pub use colors::Colors;
pub use mapping::ChoreographyModelMapper;
pub use models::{
    ChoreographyModel, DancerModel, FloorModel, PositionModel, RoleModel, SceneModel, SettingsModel,
};
pub use preferences::SettingsPreferenceKeys;
