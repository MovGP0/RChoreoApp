#[derive(Debug, Clone, PartialEq)]
pub enum TimeAction {
    InitializeCurrentTime,
    SetTimestampInput { value: String },
    ParseTimestampInput,
    SetSeconds { value: f64 },
}
