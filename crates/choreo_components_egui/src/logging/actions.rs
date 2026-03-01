#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoggingAction {
    Initialize,
    BehaviorActivated { name: String, view_model: String },
    RecordDebug { message: String },
    RecordInfo { message: String },
    RecordWarn { message: String },
    RecordError { message: String },
    SetMaxEntries { max_entries: usize },
    ClearEntries,
}
