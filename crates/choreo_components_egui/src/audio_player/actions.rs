#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AudioPlayerAction {
    Initialize,
    ToggleFlag { key: String },
}
