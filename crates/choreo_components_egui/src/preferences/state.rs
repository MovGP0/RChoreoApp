use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PreferenceWriteIntent {
    SetString {
        key: String,
        value: String,
    },
    SetBool {
        key: String,
        value: bool,
    },
    Remove {
        key: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PreferencesState {
    pub app_name: String,
    pub strings: BTreeMap<String, String>,
    pub bools: BTreeMap<String, bool>,
    pub pending_writes: Vec<PreferenceWriteIntent>,
}

impl PreferencesState {
    pub fn get_string(&self, key: &str, default_value: &str) -> String {
        self.strings
            .get(key)
            .cloned()
            .unwrap_or_else(|| default_value.to_string())
    }

    pub fn get_bool(&self, key: &str, default_value: bool) -> bool {
        self.bools.get(key).copied().unwrap_or(default_value)
    }

    pub fn scoped_key(&self, key: &str) -> String {
        if self.app_name.trim().is_empty() {
            return key.to_string();
        }
        format!("{}.{}", self.app_name, key)
    }
}
