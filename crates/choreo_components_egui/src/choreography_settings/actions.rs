#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChoreographySettingsAction {
    Initialize,
    ToggleFlag { key: String },
}
