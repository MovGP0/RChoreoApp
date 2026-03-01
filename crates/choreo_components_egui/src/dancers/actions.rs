#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DancersAction {
    Initialize,
    ToggleFlag { key: String },
}
