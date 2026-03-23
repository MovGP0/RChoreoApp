pub use super::actions::OpenAudioRequested;
pub use super::actions::OpenChoreoRequested;
pub use super::actions::OpenSvgFileCommand;
use crate::observability::TraceContext;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CloseDialogCommand;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenImageRequested {
    pub file_path: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShowDialogCommand {
    pub content: Option<String>,
}

impl OpenAudioRequested {
    #[must_use]
    pub fn from_parts(file_path: impl Into<String>, trace_context: Option<TraceContext>) -> Self {
        Self {
            file_path: file_path.into(),
            trace_context,
        }
    }
}
