#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScenesAction {
    Initialize,
    ToggleFlag { key: String },
}
