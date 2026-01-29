use log::warn;
use web_sys::Storage;

use super::{InMemoryPreferences, Preferences};

#[derive(Clone)]
pub struct WasmPreferences {
    app_name: String,
    storage: Option<Storage>,
    fallback: InMemoryPreferences,
}

impl WasmPreferences {
    pub fn new(app_name: &str) -> Self
    {
        let storage = web_sys::window()
            .and_then(|window| window.local_storage().ok())
            .flatten();

        Self {
            app_name: app_name.to_string(),
            storage,
            fallback: InMemoryPreferences::new(),
        }
    }

    fn key_for(&self, key: &str) -> String
    {
        format!("{}.{}", self.app_name, key)
    }
}

impl Preferences for WasmPreferences {
    fn get_string(&self, key: &str, default_value: &str) -> String
    {
        let Some(storage) = self.storage.as_ref() else {
            return self.fallback.get_string(key, default_value);
        };

        let full_key = self.key_for(key);
        match storage.get_item(&full_key) {
            Ok(Some(value)) => value,
            Ok(None) => default_value.to_string(),
            Err(err) => {
                warn!("Failed to read preferences key {full_key}: {:?}", err);
                default_value.to_string()
            }
        }
    }

    fn set_string(&self, key: &str, value: String)
    {
        let Some(storage) = self.storage.as_ref() else {
            self.fallback.set_string(key, value);
            return;
        };

        let full_key = self.key_for(key);
        if let Err(err) = storage.set_item(&full_key, &value) {
            warn!("Failed to write preferences key {full_key}: {:?}", err);
        }
    }

    fn remove(&self, key: &str)
    {
        if let Some(storage) = self.storage.as_ref() {
            let full_key = self.key_for(key);
            if let Err(err) = storage.remove_item(&full_key) {
                warn!("Failed to remove preferences key {full_key}: {:?}", err);
            }
        }

        self.fallback.remove(key);
    }

    fn get_bool(&self, key: &str, default_value: bool) -> bool
    {
        let text_default = if default_value { "true" } else { "false" };
        let value = self.get_string(key, text_default);
        value.eq_ignore_ascii_case("true")
    }

    fn set_bool(&self, key: &str, value: bool)
    {
        self.set_string(key, value.to_string());
    }
}
