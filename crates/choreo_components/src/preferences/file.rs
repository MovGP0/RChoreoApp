use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use log::warn;
use serde::{Deserialize, Serialize};

use super::Preferences;

#[derive(Debug, Default, Serialize, Deserialize)]
struct PreferencesState {
    strings: HashMap<String, String>,
    bools: HashMap<String, bool>,
}

#[derive(Clone)]
pub struct FilePreferences {
    state: Rc<RefCell<PreferencesState>>,
    path: PathBuf,
}

impl FilePreferences {
    pub fn new(app_name: &str) -> Self {
        let path = resolve_preferences_path(app_name);
        let state = load_state(&path).unwrap_or_default();
        Self {
            state: Rc::new(RefCell::new(state)),
            path,
        }
    }

    fn save(&self) {
        if let Err(err) = save_state(&self.path, &self.state.borrow()) {
            warn!("Failed to save preferences: {err}");
        }
    }
}

impl Preferences for FilePreferences {
    fn get_string(&self, key: &str, default_value: &str) -> String {
        self.state
            .borrow()
            .strings
            .get(key)
            .cloned()
            .unwrap_or_else(|| default_value.to_string())
    }

    fn set_string(&self, key: &str, value: String) {
        self.state
            .borrow_mut()
            .strings
            .insert(key.to_string(), value);
        self.save();
    }

    fn remove(&self, key: &str) {
        let mut state = self.state.borrow_mut();
        state.strings.remove(key);
        state.bools.remove(key);
        drop(state);
        self.save();
    }

    fn get_bool(&self, key: &str, default_value: bool) -> bool {
        self.state
            .borrow()
            .bools
            .get(key)
            .copied()
            .unwrap_or(default_value)
    }

    fn set_bool(&self, key: &str, value: bool) {
        self.state.borrow_mut().bools.insert(key.to_string(), value);
        self.save();
    }
}

fn resolve_preferences_path(app_name: &str) -> PathBuf {
    let base_dir = dirs::config_dir()
        .or_else(|| std::env::current_dir().ok())
        .unwrap_or_else(|| PathBuf::from("."));

    base_dir.join(app_name).join("preferences.json")
}

fn load_state(path: &Path) -> Option<PreferencesState> {
    let content = fs::read_to_string(path).ok()?;
    serde_json::from_str(&content).ok()
}

fn save_state(path: &Path, state: &PreferencesState) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let json = serde_json::to_string_pretty(state).map_err(std::io::Error::other)?;
    let temp_path = path.with_extension("tmp");
    fs::write(&temp_path, json)?;

    if path.exists() {
        fs::remove_file(path)?;
    }

    fs::rename(temp_path, path)?;
    Ok(())
}
