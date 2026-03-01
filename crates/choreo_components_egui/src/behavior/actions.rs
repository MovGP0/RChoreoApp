#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BehaviorAction {
    Initialize,
    ToggleFlag { key: String },
}
