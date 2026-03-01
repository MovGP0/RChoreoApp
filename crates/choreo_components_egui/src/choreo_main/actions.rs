#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChoreoMainAction {
    Initialize,
    ToggleFlag { key: String },
}
