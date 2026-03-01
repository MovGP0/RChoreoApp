#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShellAction {
    Initialize,
    ToggleFlag { key: String },
}
