#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PreferencesAction {
    Initialize,
    ToggleFlag { key: String },
}
