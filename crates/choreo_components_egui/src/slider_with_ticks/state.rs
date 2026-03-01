use std::collections::BTreeMap;

#[derive(Debug, Clone, Default)]
pub struct SliderWithTicksState {
    pub flags: BTreeMap<String, bool>,
}

impl SliderWithTicksState {
    #[must_use]
    pub fn with_flag(mut self, key: impl Into<String>, value: bool) -> Self {
        self.flags.insert(key.into(), value);
        self
    }
}
