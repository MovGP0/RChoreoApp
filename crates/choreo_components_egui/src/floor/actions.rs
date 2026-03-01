#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FloorAction {
    Initialize,
    ToggleFlag { key: String },
}
