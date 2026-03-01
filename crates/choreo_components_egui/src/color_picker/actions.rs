#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ColorPickerAction {
    Initialize,
    ToggleFlag { key: String },
}
