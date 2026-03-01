#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogEntry {
    pub level: LogLevel,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoggingState {
    pub entries: Vec<LogEntry>,
    pub max_entries: usize,
    pub dropped_entries: u64,
}

impl Default for LoggingState {
    fn default() -> Self {
        Self {
            entries: Vec::new(),
            max_entries: 200,
            dropped_entries: 0,
        }
    }
}
