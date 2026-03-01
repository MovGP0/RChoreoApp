use std::collections::BTreeMap;

#[derive(Debug, Clone, Default)]
pub struct ObservabilityState {
    pub flags: BTreeMap<String, bool>,
}

impl ObservabilityState {
    #[must_use]
    pub fn with_flag(mut self, key: impl Into<String>, value: bool) -> Self {
        self.flags.insert(key.into(), value);
        self
    }
}
