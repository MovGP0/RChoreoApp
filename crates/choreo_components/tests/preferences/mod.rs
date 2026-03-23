pub use choreo_components::preferences;

#[cfg(not(target_arch = "wasm32"))]
pub mod file_preferences_spec;
pub mod in_memory_preferences_spec;
pub mod public_api_spec;
pub mod shared_preferences_spec;
