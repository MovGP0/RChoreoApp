#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TimestampStateMachineAction {
    Initialize,
    ToggleFlag { key: String },
}
