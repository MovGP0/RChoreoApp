#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoggingAction {
    Initialize,
    ToggleFlag { key: String },
}
