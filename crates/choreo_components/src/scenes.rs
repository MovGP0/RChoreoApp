#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SceneViewModel {
    pub name: String,
}

impl SceneViewModel {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}
