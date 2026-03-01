#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TimeAction {
    Initialize,
    ToggleFlag { key: String },
}
