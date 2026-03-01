#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NavBarAction {
    Initialize,
    ToggleFlag { key: String },
}
