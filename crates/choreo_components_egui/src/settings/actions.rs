#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SettingsAction {
    Initialize,
    ToggleFlag { key: String },
}
