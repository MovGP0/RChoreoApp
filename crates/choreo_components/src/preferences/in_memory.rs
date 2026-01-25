use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use super::Preferences;

#[derive(Clone, Default)]
pub struct InMemoryPreferences {
    strings: Rc<RefCell<HashMap<String, String>>>,
    bools: Rc<RefCell<HashMap<String, bool>>>,
}

impl InMemoryPreferences {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Preferences for InMemoryPreferences {
    fn get_string(&self, key: &str, default_value: &str) -> String {
        self.strings
            .borrow()
            .get(key)
            .cloned()
            .unwrap_or_else(|| default_value.to_string())
    }

    fn set_string(&self, key: &str, value: String) {
        self.strings.borrow_mut().insert(key.to_string(), value);
    }

    fn remove(&self, key: &str) {
        self.strings.borrow_mut().remove(key);
        self.bools.borrow_mut().remove(key);
    }

    fn get_bool(&self, key: &str, default_value: bool) -> bool {
        self.bools
            .borrow()
            .get(key)
            .copied()
            .unwrap_or(default_value)
    }

    fn set_bool(&self, key: &str, value: bool) {
        self.bools.borrow_mut().insert(key.to_string(), value);
    }
}
