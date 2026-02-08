#[cfg(not(target_arch = "wasm32"))]
mod file;
mod in_memory;
mod shared;
mod types;
#[cfg(target_arch = "wasm32")]
mod wasm;

#[cfg(not(target_arch = "wasm32"))]
pub use file::FilePreferences;
pub use in_memory::InMemoryPreferences;
pub use shared::SharedPreferences;
pub use types::Preferences;
#[cfg(target_arch = "wasm32")]
pub use wasm::WasmPreferences;

#[cfg(target_arch = "wasm32")]
pub type PlatformPreferences = WasmPreferences;
#[cfg(not(target_arch = "wasm32"))]
pub type PlatformPreferences = FilePreferences;
