#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HapticsAction {
    Initialize,
    ToggleFlag { key: String },
}
