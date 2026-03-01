#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ObservabilityAction {
    Initialize,
    ToggleFlag { key: String },
}
