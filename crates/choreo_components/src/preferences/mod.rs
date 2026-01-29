mod in_memory;
mod types;
#[cfg(not(target_arch = "wasm32"))]
mod file;
#[cfg(target_arch = "wasm32")]
mod wasm;

pub use in_memory::InMemoryPreferences;
#[cfg(not(target_arch = "wasm32"))]
pub use file::FilePreferences;
#[cfg(target_arch = "wasm32")]
pub use wasm::WasmPreferences;
pub use types::Preferences;

#[cfg(target_arch = "wasm32")]
pub type PlatformPreferences = WasmPreferences;
#[cfg(not(target_arch = "wasm32"))]
pub type PlatformPreferences = FilePreferences;
