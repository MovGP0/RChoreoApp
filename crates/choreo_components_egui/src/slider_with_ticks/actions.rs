#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SliderWithTicksAction {
    Initialize,
    ToggleFlag { key: String },
}
