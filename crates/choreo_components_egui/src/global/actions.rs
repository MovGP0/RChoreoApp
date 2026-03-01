#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GlobalAction {
    Initialize,
    ToggleFlag { key: String },
}
