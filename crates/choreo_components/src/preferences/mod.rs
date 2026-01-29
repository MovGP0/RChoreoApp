mod in_memory;
mod types;
#[cfg(not(target_arch = "wasm32"))]
mod file;

pub use in_memory::InMemoryPreferences;
#[cfg(not(target_arch = "wasm32"))]
pub use file::FilePreferences;
pub use types::Preferences;
